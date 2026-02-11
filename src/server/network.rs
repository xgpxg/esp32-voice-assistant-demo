use crate::config::Config;
use crate::json_body;
use crate::server::request;
use crate::server::response::{Res, WifiListRes};
use anyhow::bail;
use embedded_svc::http::server::Request;
use embedded_svc::http::Method;
use embedded_svc::wifi::{AuthMethod, Wifi};
use esp_idf_svc::http::server::{EspHttpConnection, EspHttpServer};
use esp_idf_svc::nvs::EspDefaultNvs;
use esp_idf_svc::wifi::EspWifi;
use std::sync::{Arc, Mutex};
use std::thread::sleep;

pub fn register(
    server: &mut EspHttpServer,
    wifi: Arc<Mutex<EspWifi<'static>>>,
    nvs: Arc<Mutex<EspDefaultNvs>>,
    config: Arc<Mutex<Config>>,
) -> anyhow::Result<()> {
    // 获取wifi列表
    let wifi_clone = wifi.clone();
    server.fn_handler("/api/wifi/list", Method::Get, move |request| {
        let mut wifi_guard = wifi_clone.lock().unwrap();
        match list_all_wifi(&mut *wifi_guard) {
            Ok(list) => Res::success(list).response_to(request),
            Err(e) => Res::<()>::error(&e.to_string()).response_to(request),
        }
        Ok::<(), anyhow::Error>(())
    })?;

    // 连接wifi
    let wifi_clone = wifi.clone();
    let config_clone = config.clone();
    server.fn_handler("/api/wifi/connect", Method::Post, move |mut request| {
        let mut wifi_guard = wifi_clone.lock().unwrap();
        let mut nvs_guard = nvs.lock().unwrap();
        let mut config_guard = config_clone.lock().unwrap();
        match connect_wifi(
            &mut request,
            &mut *wifi_guard,
            &mut *nvs_guard,
            &mut *config_guard,
        ) {
            Ok(()) => Res::success(()).response_to(request),
            Err(e) => Res::<()>::error(&e.to_string()).response_to(request),
        }
        Ok::<(), anyhow::Error>(())
    })?;

    // 检查wifi是否已连接
    let wifi_clone = wifi.clone();
    server.fn_handler("/api/wifi/is_connected", Method::Get, move |request| {
        let wifi_guard = wifi_clone.lock().unwrap();
        match wifi_guard.is_connected() {
            Ok(true) => Res::success(Some(
                wifi_guard
                    .get_configuration()
                    .unwrap()
                    .as_mixed_conf_mut()
                    .0
                    .ssid
                    .to_string(),
            ))
            .response_to(request),
            Ok(false) => Res::success(()).response_to(request),
            Err(e) => Res::<()>::error(&e.to_string()).response_to(request),
        }
        Ok::<(), anyhow::Error>(())
    })?;

    Ok(())
}
pub fn list_all_wifi(wifi: &mut EspWifi) -> anyhow::Result<Vec<WifiListRes>> {
    wifi.stop_scan()?;
    let list = wifi
        .scan()?
        .into_iter()
        .map(|item| WifiListRes {
            ssid: item.ssid.to_string(),
            signal_strength: item.signal_strength,
            auth_method: item
                .auth_method
                .map(|auth_method| match auth_method {
                    AuthMethod::None => "Open".to_string(),
                    AuthMethod::WEP => "WEP".to_string(),
                    AuthMethod::WPA => "WPA".to_string(),
                    AuthMethod::WPA2Personal => "WPA2".to_string(),
                    AuthMethod::WPAWPA2Personal => "WPA/WPA2".to_string(),
                    AuthMethod::WPA2Enterprise => "WPA2-Enterprise".to_string(),
                    AuthMethod::WPA3Personal => "WPA3".to_string(),
                    AuthMethod::WPA2WPA3Personal => "WPA2/WPA3".to_string(),
                    AuthMethod::WAPIPersonal => "WAPI".to_string(),
                })
                .unwrap_or("Unknown".to_string()),
        })
        .collect();
    Ok(list)
}

fn connect_wifi(
    request: &mut Request<&mut EspHttpConnection>,
    wifi: &mut EspWifi,
    nvs: &mut EspDefaultNvs,
    config: &mut Config,
) -> anyhow::Result<()> {
    let req: request::ConnectWifiReq = json_body!(request);

    log::info!("正在连接: {}", req.ssid);

    wifi.disconnect()?;

    // 等待断开
    loop {
        if !wifi.is_connected()? {
            break;
        }
        sleep(std::time::Duration::from_millis(100));
    }

    // 重新设置SSID和密码
    let mut configuration = wifi.get_configuration()?;
    let wifi_config = configuration.as_mixed_conf_mut();
    wifi_config.0.ssid = req.ssid.as_str().try_into().unwrap();
    wifi_config.0.password = req.password.as_str().try_into().unwrap();
    wifi.set_configuration(&configuration)?;

    // 尝试连接
    wifi.connect().map_err(|e| {
        log::error!("Wifi连接失败: {:?}", e);
        e
    })?;

    let mut count = 0;
    while count < 10 {
        if wifi.is_connected()? {
            config.set_wifi_ssid(req.ssid.as_str(), nvs)?;
            config.set_wifi_password(req.password.as_str(), nvs)?;
            // nvs.set_str("WIFI_SSID", req.ssid.as_str())?;
            // nvs.set_str("WIFI_PASSWORD", req.password.as_str())?;
            log::info!("Wifi连接成功: {}", req.ssid);
            return Ok(());
        }
        sleep(std::time::Duration::from_secs(1));
        count += 1;
    }

    bail!(format!("无法连接到 {}", req.ssid))
}
