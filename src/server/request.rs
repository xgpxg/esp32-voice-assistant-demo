use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ConnectWifiReq {
    pub ssid: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct SettingsReq {
    /// 角色提示词
    pub role_prompt: String,
    /// 音色
    pub voice: String,
    /// 语速，取值：[0,255]
    pub speech_speed: u8,
    /// 音量，取值：[0,255]
    pub volume: u8,
}
