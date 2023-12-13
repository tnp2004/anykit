// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use app::minilink::link;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![rust_say_hi, short_link])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn rust_say_hi() -> String {
    "Hello from Rust!".to_string()
}

#[tauri::command]
fn short_link(url: String) -> String {
    link::get_short_link(url)
}