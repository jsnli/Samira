// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![allow(non_snake_case)]

mod database;
mod state;
mod steam;

use database::App;
use state::{AppState, ServiceAccess};
use tauri::{AppHandle, Manager, State};

#[tokio::main]
async fn main() {
    tauri::Builder::default()
        .manage(AppState {
            db: Default::default(),
            client: Default::default(),
        })
        .invoke_handler(tauri::generate_handler![
            cmd_request_data,
            cmd_populate_data,
            cmd_query_id,
            cmd_query_name,
            cmd_start,
            cmd_load_achievements,
        ])
        .setup(|app| {
            let handle = app.handle();

            let app_state: State<AppState> = handle.state();

            let db = database::init_db().expect("Failed to open database connection");

            *app_state.db.lock().unwrap() = Some(db);

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
async fn cmd_request_data(_app_handle: AppHandle) -> Vec<App> {
    let mut applist: Vec<App> = Vec::new();

    match database::request_data().await {
        Ok(app) => {
            applist.extend(app);
        }
        Err(e) => {
            eprintln!("Error: {}", e);
        }
    }

    applist
}

#[tauri::command]
async fn cmd_populate_data(app_handle: AppHandle, apps: Vec<App>) {
    let _ = app_handle.db_mut(|db| database::populate_data(db, apps));
}

#[tauri::command]
async fn cmd_query_id(app_handle: AppHandle, appid: i32) -> App {
    match app_handle.db_mut(|db| database::query_id(db, appid)) {
        Ok(app) => app,
        Err(e) => App {
            appid: 0,
            name: e.to_string(),
            last_modified: 0,
            price_change_number: 0,
        },
    }
}

#[tauri::command]
async fn cmd_query_name(app_handle: AppHandle, name: String) -> Vec<App> {
    match app_handle.db_mut(|db| database::query_name(db, name)) {
        Ok(applist) => applist,
        Err(e) => {
            eprintln!("Error: {}", e);
            let empty: Vec<App> = Vec::new();
            empty
        }
    }
}

#[tauri::command]
fn cmd_start(app_handle: AppHandle) {
    let state: State<AppState> = app_handle.state();
    *state.client.lock().unwrap() = Some(steam::start(1245620));
}

#[tauri::command]
fn cmd_load_achievements(app_handle: AppHandle) {
    let state: State<AppState> = app_handle.state();
    let client = state.client.lock().unwrap().clone();

    match client {
        Some(client) => {
            println!("Client found");
            steam::load_achievements(client)
        }
        None => {
            println!("No Client Found");
        }
    }
        
     
}
