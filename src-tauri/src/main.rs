// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![allow(non_snake_case)]

use steamworks::{Client,AppId};
use serde::{Deserialize, Serialize};
use reqwest::Error;

#[tauri::command]
fn get_steam_id() -> String {
    let (_client, _single) = Client::init_app(AppId(480)).unwrap();
    let _steam_id = _client.user().steam_id().raw().to_string();
    _steam_id
}

#[tokio::main]
async fn main() {
     
    let _ = get_steam_app_list().await;
    let steamid = get_steam_id();
    println!("{}", steamid)

    // tauri::Builder::default()
    //     .invoke_handler(tauri::generate_handler![get_steam_id])
    //     .run(tauri::generate_context!())
    //     .expect("error while running tauri application");
}

#[derive(Serialize, Deserialize, Debug)]
struct Entry {
    userId: i32,
    id: i32,
    title: String,
    body: String,
}

async fn get_steam_app_list() -> Result<(), Error> {
    let _url = "https://raw.githubusercontent.com/jsnli/steamappidlist/master/data/games_appid.json";
    let _response = reqwest::get(_url).await?;

    if _response.status().is_success() {
        let entries: Vec<Entry> = _response.json().await?;
        
        for (_index, entry) in entries.iter().enumerate() {
            println!("Entry: {:?}", entry.id);
            println!();
            println!();
        } 
            
    } else {
        println!("Failed: {}", _response.status());
        
    }
    

    println!("Hello, world!");
    Ok(())
}
