// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use steamworks::{Client,AppId};

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn get_steam_id() -> String {
    let (_client, _single) = Client::init_app(AppId(480)).unwrap();
    let _steam_id = _client.user().steam_id().raw().to_string();
    _steam_id
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_steam_id])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
