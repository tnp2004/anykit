// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use urlshortener::{ client::UrlShortener, providers::Provider };

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![rust_say_hi, mini_link])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn rust_say_hi() -> String {
    "Hello from Rust!".to_string()
}

#[tauri::command]
fn mini_link(url: String) -> String {
    let url_shortener = UrlShortener::new().unwrap();
    url_shortener.generate(url, &Provider::IsGd).unwrap()
}