// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use app::minilink::{link, qrcode};

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![short_link, qrcode])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

/* Minilink */
#[tauri::command]
fn short_link(url: String) -> String {
    match link::get_short_link(url) {
        Ok(link) => link,
        Err(e) => e.to_string(),
    }
}

#[tauri::command]
fn qrcode(url: String) -> String {
    match qrcode::get_qrcode(url) {
        Ok(link) => link,
        Err(e) => e.to_string(),
    }
}
/* --------------------------------------------- */