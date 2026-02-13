use esp_idf_svc::nvs::EspDefaultNvs;
use esp_idf_svc::wifi::{EspWifi, WifiDeviceId};
use std::sync::{Arc, OnceLock, RwLock, RwLockReadGuard, RwLockWriteGuard};

#[derive(Debug)]
pub struct Config {
    /// 设备SN
    #[allow(unused)]
    pub sn: String,
    /// WIFI名称
    pub wifi_ssid: String,
    /// WIFI密码
    pub wifi_password: String,
    /// API KEY
    pub api_key: String,
    /// 语言模型
    pub text_model: String,
    /// 角色提示词
    pub role_prompt: String,
    /// 音色
    pub voice: String,
    /// 语速，取值：[0.5 ,2.0]
    pub speech_speed: f32,
    /// 音量，取值：[0,100]
    pub volume: u8,
}

macro_rules! impl_str_setter {
    ($setter: ident,$field:ident, $key:expr) => {
        pub fn $setter(&mut self, value: &str, nvs: &mut EspDefaultNvs) -> anyhow::Result<()> {
            self.$field = value.to_string();
            nvs.set_str($key, value)?;
            Ok(())
        }
    };
}
macro_rules! impl_u8_setter {
    ($setter: ident,$field:ident, $key:expr) => {
        pub fn $setter(&mut self, value: u8, nvs: &mut EspDefaultNvs) -> anyhow::Result<()> {
            self.$field = value;
            nvs.set_u8($key, value)?;
            Ok(())
        }
    };
}
impl Config {
    pub fn new(wifi: &EspWifi, nvs: &EspDefaultNvs) -> anyhow::Result<Self> {
        let wifi_ssid = Self::get_nvs_string(nvs, "wifi_ssid", "", 128)?;
        let wifi_password = Self::get_nvs_string(nvs, "wifi_password", "", 128)?;
        let api_key = Self::get_nvs_string(nvs, "api_key", "", 128)?;
        let text_model = Self::get_nvs_string(nvs, "text_model", "qwen-plus", 128)?;
        let role_prompt = Self::get_nvs_string(nvs, "role_prompt", "", 128)?;
        let voice = Self::get_nvs_string(nvs, "voice", "longanhuan", 128)?;
        let speech_speed = Self::get_nvs_string(nvs, "speech_speed", "1.0", 128)?.parse()?;
        let volume = Self::get_nvs_u8(nvs, "volume", 50)?;
        Ok(Config {
            sn: Self::make_sn(&wifi)?,
            wifi_ssid,
            wifi_password,
            api_key,
            text_model,
            role_prompt,
            voice,
            speech_speed,
            volume,
        })
    }

    fn get_nvs_string(
        nvs: &EspDefaultNvs,
        key: &str,
        default: &str,
        buf_size: usize,
    ) -> anyhow::Result<String> {
        let mut buf = vec![0u8; buf_size];
        Ok(nvs.get_str(key, &mut buf)?.unwrap_or(default).to_string())
    }
    fn get_nvs_u8(nvs: &EspDefaultNvs, key: &str, default: u8) -> anyhow::Result<u8> {
        Ok(nvs.get_u8(key)?.unwrap_or(default))
    }
    fn get_mac_address(wifi: &EspWifi) -> Result<[u8; 6], anyhow::Error> {
        let mac = wifi.get_mac(WifiDeviceId::Sta)?;
        Ok(mac)
    }
    fn make_sn(wifi: &EspWifi) -> Result<String, anyhow::Error> {
        let mac = Self::get_mac_address(wifi)?;
        let sn = format!(
            "{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}",
            mac[0], mac[1], mac[2], mac[3], mac[4], mac[5]
        );
        Ok(sn.to_uppercase())
    }

    pub fn get_wifi_ssid(nvs: &EspDefaultNvs) -> String {
        nvs.get_str("wifi_ssid", &mut [0u8; 256])
            .unwrap()
            .unwrap_or("")
            .to_string()
    }
    pub fn get_wifi_password(nvs: &EspDefaultNvs) -> String {
        nvs.get_str("wifi_password", &mut [0u8; 256])
            .unwrap()
            .unwrap_or("")
            .to_string()
    }

    pub fn set_speech_speed(&mut self, value: f32, nvs: &mut EspDefaultNvs) -> anyhow::Result<()> {
        self.speech_speed = value;
        nvs.set_str("speech_speed", &value.to_string())?;
        Ok(())
    }
}
#[allow(unused)]
impl Config {
    impl_str_setter!(set_wifi_ssid, wifi_ssid, "wifi_ssid");
    impl_str_setter!(set_wifi_password, wifi_password, "wifi_password");
    impl_str_setter!(set_api_key, api_key, "api_key");
    impl_str_setter!(set_text_model, text_model, "text_model");
    impl_str_setter!(set_role_prompt, role_prompt, "role_prompt");
    impl_str_setter!(set_voice, voice, "voice");
    impl_u8_setter!(set_volume, volume, "volume");
}

pub static CONFIG: OnceLock<Arc<RwLock<Config>>> = OnceLock::new();

impl Config {
    pub fn get() -> RwLockReadGuard<'static, Config> {
        CONFIG.get().unwrap().read().unwrap()
    }
    pub fn get_mut() -> RwLockWriteGuard<'static, Config> {
        CONFIG.get().unwrap().write().unwrap()
    }
}
