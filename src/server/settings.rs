use crate::config::Config;
use crate::json_body;
use crate::server::request::SettingsReq;
use crate::server::response::{Res, SettingsRes};
use embedded_svc::http::Method;
use esp_idf_svc::http::server::EspHttpServer;
use esp_idf_svc::nvs::EspDefaultNvs;
use esp_idf_svc::wifi::EspWifi;
use std::sync::{Arc, Mutex};

pub fn register(
    server: &mut EspHttpServer,
    _wifi: Arc<Mutex<EspWifi<'static>>>,
    nvs: Arc<Mutex<EspDefaultNvs>>,
) -> anyhow::Result<()> {
    server.fn_handler("/api/settings/get", Method::Get, move |request| {
        let config = Config::get();
        let setting = SettingsRes {
            api_key: config.api_key.clone(),
            role_prompt: config.role_prompt.clone(),
            voice: config.voice.clone(),
            speech_speed: config.speech_speed,
            volume: config.volume,
        };
        Res::success(setting).response_to(request);
        Ok::<(), anyhow::Error>(())
    })?;

    let nvs_clone = nvs.clone();
    server.fn_handler("/api/settings/upsert", Method::Post, move |mut request| {
        let req: SettingsReq = json_body!(request);
        let mut nvs = nvs_clone.lock().unwrap();
        let mut config = Config::get_mut();
        config.set_api_key(&req.api_key, &mut nvs)?;
        config.set_role_prompt(&req.role_prompt, &mut nvs)?;
        config.set_voice(&req.voice, &mut nvs)?;
        config.set_speech_speed(req.speech_speed, &mut nvs)?;
        config.set_volume(req.volume, &mut nvs)?;
        log::info!("配置已更新：{:?}", config);
        Res::success(()).response_to(request);
        Ok::<(), anyhow::Error>(())
    })?;
    Ok(())
}
