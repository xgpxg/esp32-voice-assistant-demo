mod client;
mod config;
mod server;

use crate::client::llm::LLM;
use crate::client::mic::{Mic, MicEvent};
use crate::client::speaker::Speaker;
use crate::client::stt::STT;
use crate::client::tts::TTS;
use crate::config::{Config, CONFIG};
use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use embassy_sync::channel::{Channel, Receiver, Sender};
use embassy_time::Timer;
use esp_idf_hal::task::block_on;
use esp_idf_svc::eventloop::EspSystemEventLoop;
use esp_idf_svc::hal::peripherals::Peripherals;
use esp_idf_svc::http::server::{Configuration, EspHttpServer};
use esp_idf_svc::nvs::{EspDefaultNvs, EspDefaultNvsPartition, EspNvs};
use esp_idf_svc::wifi;
use esp_idf_svc::wifi::{AccessPointConfiguration, AuthMethod, ClientConfiguration, EspWifi};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex, RwLock};

const SSID: &str = "ESP32-WIFI";
const PASSWORD: &str = "12345678";
const CHANNEL: u8 = 1;
const DEFAULT_NS: &str = "config";

type MicChannel = Channel<CriticalSectionRawMutex, Vec<u8>, 1>;
type MicSender = Sender<'static, CriticalSectionRawMutex, Vec<u8>, 1>;
type MicReceiver = Receiver<'static, CriticalSectionRawMutex, Vec<u8>, 1>;
type SttChannel = Channel<CriticalSectionRawMutex, String, 1>;
type SttSender = Sender<'static, CriticalSectionRawMutex, String, 1>;
type SttReceiver = Receiver<'static, CriticalSectionRawMutex, String, 1>;
type LlmChannel = Channel<CriticalSectionRawMutex, String, 1>;
type LlmSender = Sender<'static, CriticalSectionRawMutex, String, 1>;
type LlmReceiver = Receiver<'static, CriticalSectionRawMutex, String, 1>;
type SpeakerChannel = Channel<CriticalSectionRawMutex, Vec<u8>, 2>;
type SpeakerSender = Sender<'static, CriticalSectionRawMutex, Vec<u8>, 2>;
type SpeakerReceiver = Receiver<'static, CriticalSectionRawMutex, Vec<u8>, 2>;
static MIC_CHANNEL: MicChannel = Channel::new();
static STT_CHANNEL: SttChannel = Channel::new();
static LLM_CHANNEL: LlmChannel = Channel::new();
static MIC_PLAY_CHANNEL: SpeakerChannel = Channel::new();
static MIC_ENABLE: AtomicBool = AtomicBool::new(true);
#[embassy_executor::main]
async fn main(spawner: embassy_executor::Spawner) {
    esp_idf_svc::sys::link_patches();

    esp_idf_svc::log::EspLogger::initialize_default();

    // 获取外设
    let peripherals = Peripherals::take().expect("Failed to initialize peripherals");
    let sys_loop = EspSystemEventLoop::take().expect("Failed to initialize system loop");
    let nvs_default_partition =
        EspDefaultNvsPartition::take().expect("Failed to initialize nvs partition");

    // 获取NVS
    let nvs =
        EspNvs::new(nvs_default_partition, DEFAULT_NS, true).expect("Failed to initialize nvs");

    // 初始化WIFI
    let mut wifi: EspWifi = EspWifi::new(peripherals.modem, sys_loop.clone(), None).unwrap();
    init_wifi(&mut wifi, &nvs).unwrap();

    let wifi = Arc::new(Mutex::new(wifi));
    let nvs = Arc::new(Mutex::new(nvs));

    // 加载配置
    let config = Config::new(&wifi.lock().unwrap(), &nvs.lock().unwrap()).unwrap();
    log::info!("配置: {:?}", config);

    let _ = CONFIG.set(Arc::new(RwLock::new(config)));

    // 创建HTTP服务
    let mut server = EspHttpServer::new(&Configuration::default()).unwrap();
    // 静态文件
    server::register_static_files(&mut server).unwrap();
    // 网络相关接口
    server::network::register(&mut server, wifi.clone(), nvs.clone()).unwrap();
    // 系统设置
    server::settings::register(&mut server, wifi.clone(), nvs.clone()).unwrap();

    loop {
        if wifi.lock().unwrap().is_connected().unwrap() {
            Timer::after_secs(3).await;
            log::info!("WiFi已连接");
            break;
        }
        log::info!("等待WiFi连接");
        Timer::after_secs(1).await;
    }

    let mic = Mic::new(
        peripherals.i2s1,
        peripherals.pins.gpio5,
        peripherals.pins.gpio38,
        peripherals.pins.gpio7,
    )
    .expect("麦克风初始化失败");

    let stt = STT::new();
    let llm = LLM::new();
    let tts = TTS::new();

    let speaker = Speaker::new(
        peripherals.i2s0,
        peripherals.pins.gpio1,
        peripherals.pins.gpio4,
        peripherals.pins.gpio2,
    )
    .unwrap();

    // 麦克风 -> 音频
    let (tx1, rx1) = (MIC_CHANNEL.sender(), MIC_CHANNEL.receiver());
    spawner.spawn(mic_task(mic, tx1)).unwrap();

    // 音频 -> 文本
    let (tx2, rx2) = (STT_CHANNEL.sender(), STT_CHANNEL.receiver());
    spawner.spawn(stt_task(rx1, tx2, stt)).unwrap();

    // 文本 -> LLM回复文本
    let (tx3, rx3) = (LLM_CHANNEL.sender(), LLM_CHANNEL.receiver());
    spawner.spawn(llm_task(rx2, tx3, llm)).unwrap();

    // LLM回复文本 -> 音频
    let (tx4, rx4) = (MIC_PLAY_CHANNEL.sender(), MIC_PLAY_CHANNEL.receiver());
    spawner.spawn(tts_task(rx3, tx4, tts)).unwrap();

    // 音频播放
    spawner.spawn(mic_play_task(rx4, speaker)).unwrap();

    core::mem::forget(wifi);
    core::mem::forget(server);
}

#[embassy_executor::task]
async fn mic_task(mut mic: Mic, tx: MicSender) {
    let mut buf = Vec::new();
    while let Ok(event) = mic.read() {
        if !MIC_ENABLE.load(Ordering::Relaxed) {
            Timer::after_millis(100).await;
            continue;
        }
        match event {
            MicEvent::Start(frame) => {
                log::info!("开始说话");
                buf.extend_from_slice(&frame);
            }
            MicEvent::End(frame) => {
                log::info!("结束说话");
                buf.extend_from_slice(&frame);
                let data = buf.drain(..).collect();
                tx.send(data).await;
            }
            MicEvent::Frame(frame) => {
                buf.extend_from_slice(&frame);
            }
            MicEvent::Silence => {}
        }

        Timer::after_millis(10).await;
    }
}

#[embassy_executor::task]
async fn stt_task(rx: MicReceiver, tx: SttSender, mut stt: STT) {
    loop {
        let data = rx.receive().await;
        if data.len() < 40960 {
            log::info!("音频长度不足40KB，忽略");
            continue;
        }
        if let Ok(Some(text)) = stt.run(data).await {
            if text.trim().is_empty() {
                continue;
            }
            tx.send(text).await;
        }
    }
}

#[embassy_executor::task]
async fn llm_task(rx: SttReceiver, tx: LlmSender, mut llm: LLM) {
    loop {
        let text = rx.receive().await;
        log::info!("请求LLM: {}", text);
        let text = llm.chat(&text);
        match text {
            Ok(text) => {
                if !text.is_empty() {
                    tx.send(text).await;
                }
            }
            Err(e) => {
                log::error!("LLM异常: {}", e);
            }
        }
    }
}

#[embassy_executor::task]
async fn tts_task(rx: LlmReceiver, tx: SpeakerSender, mut tts: TTS) {
    loop {
        let text = rx.receive().await;
        let callback = move |data: &[u8]| {
            block_on(async {
                tx.send(data.to_vec()).await;
            })
        };
        log::info!("文本转语音 => {}", text);
        let _ = tts.run(text, callback).await;
    }
}

#[embassy_executor::task]
async fn mic_play_task(rx: SpeakerReceiver, mut speaker: Speaker) {
    loop {
        let frame = rx.receive().await;
        speaker.play_chunked(&frame, 1024).unwrap();
    }
}

fn init_wifi(wifi: &mut EspWifi<'static>, nvs: &EspDefaultNvs) -> anyhow::Result<()> {
    let ssid = Config::get_wifi_ssid(nvs);
    let password = Config::get_wifi_password(nvs);

    let cfg = wifi::Configuration::Mixed(
        ClientConfiguration {
            ssid: ssid.as_str().try_into().unwrap(),
            password: password.as_str().try_into().unwrap(),
            ..Default::default()
        },
        AccessPointConfiguration {
            ssid: SSID.try_into().unwrap(),
            ssid_hidden: false,
            auth_method: AuthMethod::WPA2Personal,
            password: PASSWORD.try_into().unwrap(),
            channel: CHANNEL,
            ..Default::default()
        },
    );

    wifi.set_configuration(&cfg)?;

    wifi.start()?;

    // 尝试连接
    if !ssid.is_empty() {
        log::info!("Wifi连接中: {}", ssid);
        wifi.connect()?;
    }

    Ok(())
}
