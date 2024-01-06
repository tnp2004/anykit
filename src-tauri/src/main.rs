// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use app::{minilink, tubelo};

#[derive(serde::Serialize)]
struct Response<T> {
    status: String,
    message: String,
    value: T,
}

#[tokio::main]
async fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            short_link,
            qrcode,
            download_mp3,
            download_mp4
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

/* Minilink */
#[tauri::command]
fn short_link(url: String) -> Response<String> {
     match minilink::link::get_short_link(url) {
        Ok(link) => Response {
            status: "OK".to_string(),
            message: "short link has been created".to_string(),
            value: link,
        },
        Err(e) => Response {
            status: "Error".to_string(),
            message: "something went wrong".to_string(),
            value: e.to_string(),
        },
    }
}

#[tauri::command]
fn qrcode(url: String) -> String {
    match minilink::qrcode::get_qrcode(url) {
        Ok(link) => link.to_string(),
        Err(e) => e.to_string(),
    }
}

/* Tubelo */
#[tauri::command]
async fn download_mp3(url: String) {
    let result = tokio::task::spawn_blocking(move || tubelo::convert::Downloader::init(url));
    let loader = result.await.unwrap();
    loader.mp3().await;
}

#[tauri::command]
async fn download_mp4(url: String) {
    let loader = tubelo::convert::Downloader::init(url);
    loader.mp4().await;
}
