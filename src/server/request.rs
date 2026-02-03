use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ConnectWifiReq{
    pub ssid: String,
    pub password: String
}