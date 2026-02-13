use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ConnectWifiReq {
    pub ssid: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct SettingsReq {
    /// API Key
    pub api_key: String,
    /// 角色提示词
    pub role_prompt: String,
    /// 音色
    pub voice: String,
    /// 语速，取值：[0.5,2.0]
    pub speech_speed: f32,
    /// 音量，取值：[0,100]
    pub volume: u8,
}
