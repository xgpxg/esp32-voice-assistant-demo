mod server;

use embedded_svc::http::Headers;
use esp_idf_svc::eventloop::EspSystemEventLoop;
use esp_idf_svc::hal::peripherals::Peripherals;
use esp_idf_svc::http::server::{Configuration, EspHttpServer};
use esp_idf_svc::nvs::{EspDefaultNvs, EspDefaultNvsPartition, EspNvs};
use esp_idf_svc::wifi;
use esp_idf_svc::wifi::{AccessPointConfiguration, AuthMethod, ClientConfiguration, EspWifi};
use std::sync::{Arc, Mutex};
const SSID: &str = "ESP32-WIFI";
const PASSWORD: &str = "12345678";
const CHANNEL: u8 = 1;
const DEFAULT_NS: &str = "config";

fn main() -> anyhow::Result<()> {
    esp_idf_svc::sys::link_patches();
    esp_idf_svc::log::EspLogger::initialize_default();

    // 获取外设
    let peripherals = Peripherals::take()?;
    let sys_loop = EspSystemEventLoop::take()?;
    let nvs_default_partition = EspDefaultNvsPartition::take()?;

    // 获取NVS
    let nvs = EspNvs::new(nvs_default_partition, DEFAULT_NS, true)?;

    // 初始化WIFI
    let mut wifi = EspWifi::new(peripherals.modem, sys_loop.clone(), None)?;
    init_wifi(&mut wifi, &nvs)?;

    let wifi = Arc::new(Mutex::new(wifi));
    let nvs = Arc::new(Mutex::new(nvs));

    // 创建HTTP服务
    let mut server = EspHttpServer::new(&Configuration::default())?;
    // 静态文件
    server::register_static_files(&mut server)?;
    // 网络相关接口
    server::network::register(&mut server, wifi.clone(), nvs)?;

    loop {
        std::thread::sleep(std::time::Duration::from_secs(1));
    }

    // 或者
    // core::mem::forget(wifi);
    // core::mem::forget(server);

    Ok(())
}

fn init_wifi(wifi: &mut EspWifi<'static>, nvs: &EspDefaultNvs) -> anyhow::Result<()> {
    let mut buf = [0; 512];
    let ssid = nvs.get_str("WIFI_SSID", &mut buf)?.unwrap_or_default();

    let mut buf = [0; 512];
    let password = nvs.get_str("WIFI_PASSWORD", &mut buf)?.unwrap_or_default();

    let cfg = wifi::Configuration::Mixed(
        ClientConfiguration {
            ssid: ssid.try_into().unwrap(),
            password: password.try_into().unwrap(),
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
    if ssid != "" {
        log::info!("Wifi连接中: {}", ssid);
        wifi.connect()?;
    }

    Ok(())
}
