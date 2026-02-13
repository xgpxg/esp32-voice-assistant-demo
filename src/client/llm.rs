use crate::config::Config;
use embedded_svc::{http::client::Client as HttpClient, io::Write, utils::io};
use esp_idf_svc::http::client::{Configuration, EspHttpConnection};
use serde_json::{json, Value};

pub struct LLM {
    client: embedded_svc::http::client::Client<EspHttpConnection>,
}

impl LLM {
    pub fn new() -> Self {
        let config = Configuration {
            use_global_ca_store: true,
            crt_bundle_attach: Some(esp_idf_svc::sys::esp_crt_bundle_attach),
            timeout: Some(core::time::Duration::from_secs(30)),
            buffer_size: Some(1204 * 4),
            ..Default::default()
        };

        let client = HttpClient::wrap(EspHttpConnection::new(&config).unwrap());
        LLM { client }
    }

    const API: &str = "https://dashscope.aliyuncs.com/compatible-mode/v1/chat/completions";
    //const MODEL: &str = "qwen-plus";
    //const API_KEY: &str = env!("MASTER_JIN_ALI_API_KEY");
    const DEFAULT_PROMPT: &str = "回答时禁止废话，精简标点符号，返回易于播放的文本格式。";
    pub fn chat(&mut self, input: &str) -> anyhow::Result<String> {
        let config = Config::get();
        let role_prompt = match config.role_prompt.as_str() {
            "" => Self::DEFAULT_PROMPT.to_string(),
            role_prompt => format!("{} ; {}", Self::DEFAULT_PROMPT, role_prompt),
        };
        let json = json!({
            "model": config.text_model,
            "messages": [
                {
                    "role": "system",
                    "content": role_prompt
                },
                {
                    "role": "user",
                    "content": input
                }
            ],
            "stream": false,
            "enable_search": true,
        });
        let result = self.post(serde_json::to_vec(&json).unwrap())?;
        Ok(result)
    }

    fn post(&mut self, body: Vec<u8>) -> anyhow::Result<String> {
        let content_length_header = format!("{}", body.len());
        let headers = [
            ("content-type", "application/json"),
            ("content-length", &*content_length_header),
            ("Authorization", &format!("Bearer {}", Config::get().api_key)),
        ];

        let mut request = self.client.post(Self::API, &headers)?;
        request.write_all(&body)?;
        request.flush()?;
        log::info!("-> POST {}", Self::API);
        let mut response = request.submit()?;

        let status = response.status();
        log::info!("<- {status}");
        let mut buf = [0u8; 4096];
        let bytes_read = io::try_read_full(&mut response, &mut buf).map_err(|e| e.0)?;
        log::info!("Read {bytes_read} bytes");
        match std::str::from_utf8(&buf[0..bytes_read]) {
            Ok(body_string) => {
                let json = serde_json::from_str::<Value>(body_string);
                match json {
                    Ok(json) => {
                        let message = &json["choices"][0]["message"]["content"];
                        return Ok(message.as_str().unwrap_or_default().to_string());
                    }
                    Err(e) => log::error!("Error parsing JSON: {e}"),
                }
            }
            Err(e) => log::error!("Error decoding response body: {e}"),
        };

        Ok("".to_string())
    }
}
