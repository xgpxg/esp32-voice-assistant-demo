mod request;
mod response;

use esp_idf_svc::eventloop::EspSystemEventLoop;
use esp_idf_svc::hal::peripherals::Peripherals;
use esp_idf_svc::http::server::{Configuration, EspHttpServer};
use esp_idf_svc::http::Method;
use esp_idf_svc::io::Write;
use esp_idf_svc::nvs::EspDefaultNvsPartition;
use esp_idf_svc::wifi;
use esp_idf_svc::wifi::{
    AccessPointConfiguration, AuthMethod, BlockingWifi, ClientConfiguration, EspWifi,
};
use std::sync::{Arc, Mutex};

const SSID: &str = "ESP32-WIFI";
const PASSWORD: &str = "12345678";
const CHANNEL: u8 = 1;

macro_rules! register_static_files {
    ($server:expr, $($route:expr => $file:expr),*) => {
        $(
            {
                let file_data = include_bytes!($file);
                let file_owned = Vec::from(file_data);
                $server.fn_handler($route, Method::Get, move |req| {
                    req.into_ok_response()?.write_all(&file_owned)
                })?;
            }
        )*
    };
}

fn main() -> anyhow::Result<()> {
    esp_idf_svc::sys::link_patches();
    esp_idf_svc::log::EspLogger::initialize_default();

    log::info!("Hello, world!");

    // 获取外设
    let peripherals = Peripherals::take()?;
    let sys_loop = EspSystemEventLoop::take()?;
    let nvs = EspDefaultNvsPartition::take()?;

    // 启动WIFI
    let mut wifi = BlockingWifi::wrap(
        EspWifi::new(peripherals.modem, sys_loop.clone(), Some(nvs))?,
        sys_loop,
    )?;

    connect_wifi(&mut wifi)?;

    let wifi = Arc::new(Mutex::new(wifi));

    // 创建HTTP服务器
    let mut server = EspHttpServer::new(&Configuration::default())?;

    register_static_files!(
        server,
        "/" => "pages/index.html",
        "/wifi.html" => "pages/wifi.html",
        "/settings.html" => "pages/settings.html",
        "/about.html" => "pages/about.html",
        "/styles/common.css" => "pages/styles/common.css",
        "/styles/page.css" => "pages/styles/page.css"
    );

     let wifi_for_server = wifi.clone();
    server.fn_handler("/api/wifi/list", Method::Get, move |req| {
        let mut wifi_guard = wifi_for_server.lock().unwrap();
        let wifi = wifi_list(&mut *wifi_guard).map_err(|e| {
            log::error!("wifi list error: {:?}", e);
            anyhow::Error::from(e)
        })?;
        log::info!("wifi list: {:?}", wifi);
        let _ = req
            .into_ok_response()?
            .write_all(&serde_json::to_vec(&wifi)?);
        Ok::<(), anyhow::Error>(())
    })?;

    loop {
        std::thread::sleep(std::time::Duration::from_secs(1));
    }

    // 或者
    // core::mem::forget(wifi);
    // core::mem::forget(server);

    Ok(())
}

fn connect_wifi(wifi: &mut BlockingWifi<EspWifi<'static>>) -> anyhow::Result<()> {
    log::info!("Starting Wifi...");

    let cfg = wifi::Configuration::Mixed(
        ClientConfiguration {
            ssid: SSID.try_into().unwrap(),
            password: PASSWORD.try_into().unwrap(),
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
    log::info!("Wifi started");


    // wifi.connect()?;
    // log::info!("Wifi connected");

    //wifi.wait_netif_up()?;

    log::info!("Wifi netif up");

    log::info!("Created Wifi with WIFI_SSID `{SSID}` and WIFI_PASS `{PASSWORD}`");

    Ok(())
}

pub fn wifi_list(wifi: &mut BlockingWifi<EspWifi<'static>>) -> anyhow::Result<Vec<String>> {
    Ok(wifi
        .scan()?
        .into_iter()
        .map(|x| x.ssid.to_string())
        .collect())
}
