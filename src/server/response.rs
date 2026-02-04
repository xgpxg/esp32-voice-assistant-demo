use embedded_svc::http::server::Request;
use esp_idf_svc::http::server::EspHttpConnection;
use esp_idf_svc::io::Write;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Res<T> {
    pub code: i32,
    pub msg: String,
    pub data: Option<T>,
}

/// 响应成功
const SUCCESS_CODE: i32 = 0;
/// 系统错误
const ERROR_CODE: i32 = 1;

impl<T> Res<T>
where
    T: Serialize,
{
    pub fn success(data: T) -> Self {
        Res {
            code: SUCCESS_CODE,
            msg: "".to_string(),
            data: Some(data),
        }
    }

    pub fn error(msg: &str) -> Self {
        Res {
            code: ERROR_CODE,
            msg: msg.to_string(),
            data: None,
        }
    }

    #[allow(unused)]
    pub fn is_success(&self) -> bool {
        self.code == 0
    }

    pub fn to_json_string(&self) -> String {
        serde_json::json!(&self).to_string()
    }

    pub fn to_json_vec(&self) -> Vec<u8> {
        serde_json::to_vec(&self).unwrap()
    }

    pub fn response_to(&self, request: Request<&mut EspHttpConnection>) {
        request
            .into_ok_response()
            .unwrap()
            .write_all(&self.to_json_vec())
            .unwrap();
    }
}

#[derive(Debug, Serialize)]
pub struct WifiListRes {
    /// WiFi名
    pub ssid: String,
    /// 信号强度
    pub signal_strength: i8,
    /// 认证方式
    pub auth_method: String,
}

#[derive(Debug, Serialize)]
pub struct SettingsRes {
    /// 角色提示词
    pub role_prompt: String,
    /// 音色
    pub voice: String,
    /// 语速，取值：[0,255]
    pub speech_speed: u8,
    /// 音量，取值：[0,255]
    pub volume: u8,
}
