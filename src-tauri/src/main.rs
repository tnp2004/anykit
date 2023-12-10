// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use urlshortener::{ client::UrlShortener, providers::Provider };

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![rust_say_hi])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn rust_say_hi() -> String {
    "Hello from Rust!".to_string()
}

#[tauri::command]
fn mini_link() -> String {
    let url_shortener = UrlShortener::new().unwrap();
    let long_url = "https://wetv.vip/th/play/b0i5wrv7r5stokx-%E0%B8%95%E0%B8%B3%E0%B8%99%E0%B8%B2%E0%B8%99%E0%B8%88%E0%B8%AD%E0%B8%A1%E0%B8%A2%E0%B8%B8%E0%B8%97%E0%B8%98%E0%B9%8C%E0%B8%A0%E0%B8%B9%E0%B8%95%E0%B8%96%E0%B8%B1%E0%B8%87%E0%B8%8B%E0%B8%B2%E0%B8%99%202%20(%E0%B8%9E%E0%B8%B2%E0%B8%81%E0%B8%A2%E0%B9%8C%E0%B9%84%E0%B8%97%E0%B8%A2)/f0047gmreps-EP17%3A%20%E0%B8%95%E0%B8%B3%E0%B8%99%E0%B8%B2%E0%B8%99%E0%B8%88%E0%B8%AD%E0%B8%A1%E0%B8%A2%E0%B8%B8%E0%B8%97%E0%B8%98%E0%B9%8C%E0%B8%A0%E0%B8%B9%E0%B8%95%E0%B8%96%E0%B8%B1%E0%B8%87%E0%B8%8B%E0%B8%B2%E0%B8%99%202%20(%E0%B8%9E%E0%B8%B2%E0%B8%81%E0%B8%A2%E0%B9%8C%E0%B9%84%E0%B8%97%E0%B8%A2)";

    url_shortener.generate(long_url, &Provider::IsGd).unwrap()
}