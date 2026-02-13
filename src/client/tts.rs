use crate::client::{Action, WsEvent};
use crate::config::Config;
use crate::MIC_ENABLE;
use embassy_time::Timer;
use embedded_svc::ws::FrameType;
use esp_idf_svc::ws::client::{EspWebSocketClient, EspWebSocketClientConfig, WebSocketEventType};
use serde_json::{json, Value};
use std::sync::atomic::Ordering;
use std::sync::mpsc;
use std::time::Duration;

pub struct TTS {}

impl TTS {
    pub fn new() -> Self {
        Self {}
    }

    pub async fn run(
        &mut self,
        text: String,
        mut callback: impl FnMut(&[u8]) + Send,
    ) -> anyhow::Result<()> {
        MIC_ENABLE.store(false, Ordering::SeqCst);

        let (tx, rx) = mpsc::channel();
        let mut client = Self::new_ws_client(tx)?;
        if rx.recv() == Ok(WsEvent::Connected) {
            log::info!("TTS 服务已连接");
        }

        let task_id = &uuid::Uuid::new_v4().to_string();
        log::info!("任务 {} 未开始，开始任务", task_id);
        // 发送启动任务
        client.send(
            FrameType::Text(false),
            TTSAction::run_task(&task_id).as_bytes(),
        )?;

        if rx.recv() == Ok(WsEvent::TaskStarted) {
            log::info!("任务 {} 已启动", task_id);
        }

        log::info!("正在将文本转为语音");
        let splits = text.split("。");
        for s in splits {
            client.send(
                FrameType::Text(false),
                TTSAction::continue_task(task_id, &format!("{}。", s)).as_bytes(),
            )?;
            Timer::after_millis(100).await;
        }

        Timer::after_millis(100).await;
        client.send(
            FrameType::Text(false),
            TTSAction::finish_task(&task_id).as_bytes(),
        )?;

        // recv会阻塞线程，当有两个同时阻塞时会死锁
        /*if rx.recv() == Ok(Event::TaskFinished) {
            log::info!("任务 {} 已完成", task_id);
        }*/
        loop {
            match rx.try_recv() {
                Ok(WsEvent::ResultGenerated(data)) => {
                    callback(&data);
                }
                Ok(WsEvent::TaskFinished) => {
                    log::info!("任务 {} 已完成", task_id);
                    break;
                }
                _ => {}
            }
            Timer::after_millis(10).await;
        }

        MIC_ENABLE.store(true, Ordering::SeqCst);

        drop(client);

        Timer::after_millis(10).await;

        Ok(())
    }

    const API: &str = "wss://dashscope.aliyuncs.com/api-ws/v1/inference";
    //const API_KEY: &str = env!("MASTER_JIN_ALI_API_KEY");
    fn new_ws_client<'a>(
        tx: mpsc::Sender<WsEvent<Vec<u8>>>,
    ) -> anyhow::Result<EspWebSocketClient<'a>> {
        let mut config = EspWebSocketClientConfig::default();
        // 设置认证头
        let headers = [("Authorization", &Config::get().api_key)];
        let mut headers_str = headers
            .iter()
            .map(|(k, v)| format!("{k}: {v}"))
            .collect::<Vec<_>>()
            .join("\r\n");
        headers_str.push_str("\r\n\r\n");
        config.headers = Some(&headers_str);

        config.crt_bundle_attach = Some(esp_idf_svc::sys::esp_crt_bundle_attach);
        config.use_global_ca_store = true;
        config.buffer_size = 1024 * 128;
        config.task_stack = 1024 * 16;
        config.network_timeout_ms = Duration::from_secs(60);
        config.ping_interval_sec = Duration::from_secs(5);
        config.disable_auto_reconnect = true;

        let client = unsafe {
            EspWebSocketClient::new_nonstatic(
                Self::API,
                &config,
                Duration::from_secs(10),
                move |event| match event {
                    Ok(event) => match event.event_type {
                        WebSocketEventType::Connected => {
                            tx.send(WsEvent::Connected).ok();
                        }
                        WebSocketEventType::Text(ref text) => {
                            if let Ok(json) = serde_json::from_str::<Value>(text) {
                                if json["header"]["event"].as_str() == Some("task-started") {
                                    tx.send(WsEvent::TaskStarted).ok();
                                }
                                if json["header"]["event"].as_str() == Some("task-finished") {
                                    tx.send(WsEvent::TaskFinished).ok();
                                    log::info!("任务已完成");
                                }
                            }
                        }
                        WebSocketEventType::Binary(data) => {
                            tx.send(WsEvent::ResultGenerated(data.to_vec())).ok();
                        }
                        WebSocketEventType::Disconnected => {
                            tx.send(WsEvent::Disconnected).ok();
                        }
                        _ => {}
                    },
                    Err(err) => {
                        log::error!("WebSocket 错误: {:?}", err);
                    }
                },
            )
        }
        .map_err(|e| {
            log::error!("WebSocket 错误: {:?}", e);
            e
        })?;
        Ok(client)
    }
}

struct TTSAction;
impl Action for TTSAction {
    fn run_task(task_id: &str) -> String {
        let config = Config::get();
        Self::run_task_with_payload(
            task_id,
            json!({
                "task_group": "audio",
                "task": "tts",
                "function": "SpeechSynthesizer",
                "model": "cosyvoice-v3-flash",
                "parameters": {
                    "text_type": "PlainText",
                    "voice": config.voice,
                    "format": "pcm",
                    "sample_rate": 16000,
                    "volume": config.volume,
                    "rate": config.speech_speed,
                    "pitch": 1
                },
                "input": {
                }
            }),
        )
    }
}
impl TTSAction {
    fn continue_task(task_id: &str, text: &str) -> String {
        let json = json!({
            "header": {
                "action": "continue-task",
                "task_id": task_id,
                "streaming": "duplex"
            },
            "payload": {
                "input": {
                    "text": text
                }
            }
        });
        json.to_string()
    }
}
