pub mod network;
mod request;
mod response;

use embedded_svc::http::Method;
use embedded_svc::io::Write;
use esp_idf_svc::http::server::EspHttpServer;

macro_rules! register_static_files {
    ($server:expr, $($route:expr => $file:expr),*) => {
        $(
            {
                let file_data = include_bytes!($file);
                let file_owned = Vec::from(file_data);
                let content_type = get_content_type($file);
                $server.fn_handler($route, Method::Get, move |req| {
                    req.into_response(200,None,&[("Content-Type", content_type)])?.write_all(&file_owned)
                })?;
            }
        )*
    };
}
fn get_content_type(file_path: &str) -> &'static str {
    match file_path
        .rsplit('.')
        .next()
        .unwrap_or("")
        .to_lowercase()
        .as_str()
    {
        "html" => "text/html; charset=utf-8",
        "css" => "text/css; charset=utf-8",
        "js" => "application/javascript; charset=utf-8",
        "json" => "application/json; charset=utf-8",
        "png" => "image/png",
        "jpg" | "jpeg" => "image/jpeg",
        "gif" => "image/gif",
        "svg" => "image/svg+xml",
        "ico" => "image/x-icon",
        _ => "text/plain; charset=utf-8",
    }
}

pub fn register_static_files(server: &mut EspHttpServer) -> anyhow::Result<()> {
    register_static_files!(
        server,
        "/styles/common.css" => "../../pages/styles/common.css",
        "/styles/page.css" => "../../pages/styles/page.css",
        "/scripts/vue.min.js" => "../../pages/scripts/vue.min.js",
        "/scripts/utils.js" => "../../pages/scripts/utils.js",
        "/api.js" => "../../pages/api.js",
        "/" => "../../pages/index.html"
    );
    Ok(())
}
