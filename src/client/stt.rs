use crate::client::{Action, WsEvent};
use embassy_time::Timer;
use embedded_svc::ws::FrameType;
use esp_idf_svc::ws::client::{EspWebSocketClient, EspWebSocketClientConfig, WebSocketEventType};
use serde_json::{json, Value};
use std::sync::mpsc;
use std::time::Duration;

pub struct STT;

impl STT {
    pub fn new() -> Self {
        Self
    }

    pub async fn run(&mut self, data: Vec<u8>) -> anyhow::Result<Option<String>> {
        let (tx, rx) = mpsc::channel();
        let mut client = Self::new_ws_client(tx)?;
        if rx.recv() == Ok(WsEvent::Connected) {
            log::info!("STT 服务已连接");
        }

        let task_id = &uuid::Uuid::new_v4().to_string();
        log::info!("任务 {} 未开始，开始任务", task_id);

        // 发送启动任务
        client.send(
            FrameType::Text(false),
            STTAction::run_task(&task_id).as_bytes(),
        )?;

        if rx.recv() == Ok(WsEvent::TaskStarted) {
            log::info!("任务 {} 已启动", task_id);
        }
        Timer::after_millis(100).await;

        log::info!("正在将 {} 字节的音频转为文本", data.len());
        const CHUNK_SIZE: usize = 1024 * 32;
        for chunk in data.chunks(CHUNK_SIZE) {
            client.send(FrameType::Binary(false), chunk)?;
            Timer::after_millis(50).await;
        }
        client.send(FrameType::Binary(false), &data)?;

        client.send(
            FrameType::Text(false),
            STTAction::finish_task(&task_id).as_bytes(),
        )?;

        /*if rx.recv() == Ok(WsEvent::TaskFinished) {
            log::info!("任务 {} 已完成", task_id);
        }*/

        let mut result = String::new();

        loop {
            if let Ok(event) = rx.try_recv() {
                match event {
                    WsEvent::ResultGenerated(text) => result.push_str(&text),
                    _ => {
                        break;
                    }
                }
                break;
            }
            Timer::after_millis(100).await;
        }

        drop(client);

        let result = result.trim().replace("\n", "");

        if result.is_empty() {
            log::info!("未识别到文本");
            return Ok(None);
        }

        Ok(Some(result))
    }

    const API: &str = "wss://dashscope.aliyuncs.com/api-ws/v1/inference";
    const API_KEY: &str = env!("MASTER_JIN_ALI_API_KEY");
    fn new_ws_client(
        tx: mpsc::Sender<WsEvent<String>>,
    ) -> anyhow::Result<EspWebSocketClient<'static>> {
        log::info!("创建 STT WS 客户端");

        let mut config = EspWebSocketClientConfig::default();

        let headers = [("Authorization", Self::API_KEY)];
        let mut headers_str = headers
            .iter()
            .map(|(k, v)| format!("{k}: {v}"))
            .collect::<Vec<_>>()
            .join("\r\n");
        headers_str.push_str("\r\n\r\n");
        config.headers = Some(&headers_str);

        config.crt_bundle_attach = Some(esp_idf_svc::sys::esp_crt_bundle_attach);
        config.use_global_ca_store = true;
        config.buffer_size = 1024 * 32;
        config.task_stack = 1024 * 16;

        let client =
            EspWebSocketClient::new(Self::API, &config, Duration::from_secs(10), move |event| {
                match event {
                    Ok(event) => match event.event_type {
                        WebSocketEventType::Connected => {
                            tx.send(WsEvent::Connected).unwrap();
                        }
                        WebSocketEventType::Text(ref text) => {
                            let json: Value = serde_json::from_str(text).expect("JSON 解析错误");
                            if json["header"]["event"].as_str() == Some("task-started") {
                                tx.send(WsEvent::TaskStarted).ok();
                                return;
                            }
                            if json["header"]["event"].as_str() == Some("result-generated") {
                                let sentence = &json["payload"]["output"]["transcription"];
                                if sentence["sentence_end"].as_bool() == Some(true) {
                                    let text = sentence["text"].as_str().unwrap_or_default();
                                    tx.send(WsEvent::ResultGenerated(text.to_string())).ok();
                                }
                            }
                            if json["header"]["event"].as_str() == Some("task-finished") {
                                tx.send(WsEvent::TaskFinished).ok();
                            }
                        }
                        WebSocketEventType::Disconnected => {
                            tx.send(WsEvent::Disconnected).ok();
                        }
                        _ => {}
                    },
                    Err(e) => {
                        log::error!("WebSocket 错误: {:?}", e);
                    }
                }
            })
            .map_err(|e| {
                log::error!("WebSocket 连接失败: {:?}", e);
                e
            })?;
        Ok(client)
    }
}

struct STTAction {}
impl Action for STTAction {
    fn run_task(task_id: &str) -> String {
        Self::run_task_with_payload(
            task_id,
            json!({
                "model": "gummy-chat-v1",
                "parameters": {
                    "sample_rate": 16000,
                    "format": "wav",
                    "source_language": null,
                    "transcription_enabled": true,
                    "translation_enabled": false,
                    "translation_target_languages": ["en"]
                },
                "input": {},
                "task": "asr",
                "task_group": "audio",
                "function": "recognition"
            }),
        )
    }
}
