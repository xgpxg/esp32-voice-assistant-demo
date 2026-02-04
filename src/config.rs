use esp_idf_svc::nvs::EspDefaultNvs;
use esp_idf_svc::wifi::{EspWifi, WifiDeviceId};

#[derive(Debug)]
pub struct Config {
    /// 设备SN
    pub sn: String,
    /// WIFI名称
    pub wifi_ssid: String,
    /// WIFI密码
    pub wifi_password: String,
    /// 语言模型
    pub text_model: String,
    /// 角色提示词
    pub role_prompt: String,
    /// 语音模型
    pub tts_model: String,
    /// 音色
    pub voice: String,
    /// 语速，取值：[0, 255]
    /// 映射到：[0.5 ,2.0]
    pub speech_speed: u8,
    /// 音量，取值：[0,255]
    /// 映射到：[1.0 ,10.0]
    pub volume: u8,
}

impl Config {
    pub fn default_text_model() -> &'static str {
        "qwen-plus"
    }
    pub fn default_role_prompt() -> &'static str {
        "You are a helpful assistant."
    }
    pub fn default_tts_model() -> &'static str {
        "qwen-tts"
    }
    pub fn default_voice() -> &'static str {
        ""
    }

    pub fn default_speech_speed() -> u8 {
        128
    }
    pub fn default_volume() -> u8 {
        128
    }
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
        let mut buf = [0u8; 128];
        let wifi_ssid = nvs
            .get_str("wifi_ssid", &mut buf)?
            .unwrap_or("")
            .to_string();
        let mut buf = [0u8; 128];
        let wifi_password = nvs
            .get_str("wifi_password", &mut buf)?
            .unwrap_or("")
            .to_string();
        let mut buf = [0u8; 128];
        let text_model = nvs
            .get_str("text_model", &mut buf)?
            .unwrap_or(&Self::default_text_model())
            .to_string();
        let mut buf = [0u8; 256];
        let role_prompt = nvs
            .get_str("role_prompt", &mut buf)?
            .unwrap_or(&Self::default_role_prompt())
            .to_string();
        let mut buf = [0u8; 128];
        let tts_model = nvs
            .get_str("tts_model", &mut buf)?
            .unwrap_or(&Self::default_tts_model())
            .to_string();
        let mut buf = [0u8; 128];
        let voice = nvs
            .get_str("voice", &mut buf)?
            .unwrap_or(&Self::default_voice())
            .to_string();
        let speech_speed = nvs
            .get_u8("speech_speed")?
            .unwrap_or(Self::default_speech_speed());
        let volume = nvs.get_u8("volume")?.unwrap_or(Self::default_volume());
        Ok(Config {
            sn: Self::make_sn(&wifi)?,
            wifi_ssid,
            wifi_password,
            text_model,
            role_prompt,
            tts_model,
            voice,
            speech_speed,
            volume,
        })
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
}

impl Config {
    impl_str_setter!(set_wifi_ssid, wifi_ssid, "wifi_ssid");
    impl_str_setter!(set_wifi_password, wifi_password, "wifi_password");
    impl_str_setter!(set_text_model, text_model, "text_model");
    impl_str_setter!(set_role_prompt, role_prompt, "role_prompt");
    impl_str_setter!(set_tts_model, tts_model, "tts_model");
    impl_str_setter!(set_voice, voice, "voice");
    impl_u8_setter!(set_speech_speed, speech_speed, "speech_speed");
    impl_u8_setter!(set_volume, volume, "volume");
}
