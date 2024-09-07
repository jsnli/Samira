// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![allow(non_snake_case)]

use reqwest::Error;
use rusqlite::{Connection, Result};
use serde::{Deserialize, Serialize};
use steamworks::{AppId, Client};

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
    println!("{}", steamid);

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_steam_id])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[derive(Serialize, Deserialize, Debug)]
struct App {
    appid: i32,
    name: String,
    last_modified: i32,
    price_change_number: i32,
}

async fn get_steam_app_list() -> Result<(), Error> {
    let _url =
        "https://raw.githubusercontent.com/jsnli/steamappidlist/master/data/games_appid.json";
    let _response = reqwest::get(_url).await?;

    if _response.status().is_success() {
        let apps: Vec<App> = _response.json().await?;
        let _ = create_db(apps);
        // for (_index, app) in apps.iter().enumerate() {
        //     println!("App: {:?}", app.name);
        // }
    } else {
        println!("Failed: {}", _response.status());
    }

    Ok(())
}

fn create_db(_apps: Vec<App>) -> Result<()> {
    let mut conn = Connection::open_in_memory().expect("Failed to open database connection");

    conn.execute(
        "CREATE TABLE IF NOT EXISTS apps (
            appid INTEGER PRIMARY KEY,
            name TEXT NOT NULL,
            last_modified INTEGER NOT NULL,
            price_change_number INTEGER NOT NULL
        )",
        (),
    )?;

    let _t = conn.transaction()?;
    {
        let mut _insertion_sql = _t.prepare("INSERT INTO apps (appid, name, last_modified, price_change_number) VALUES (?, ?, ?, ?)")?;

        for _app in _apps {
            let _ = _insertion_sql.execute((
                &_app.appid,
                &_app.name,
                &_app.last_modified,
                &_app.price_change_number,
            ));
        }
    }

    _t.commit()?;

    match fetch_row_by_id(&conn, 440) {

        Ok(row) => {
            println!("Found game: {:?}", row);
        },
        Err(e) => {
            eprintln!("Error: {}", e);
        }

    }

    Ok(())
}

fn fetch_row_by_id(conn: &Connection, appid: i32) -> Result<App> {
    let mut _fetch_sql = conn.prepare(
        "SELECT appid, name, last_modified, price_change_number FROM apps WHERE appid = ?",
    )?;

    let row = _fetch_sql.query_row(&[&appid], |row| {
        Ok(App {
            appid: row.get(0)?,
            name: row.get(1)?,
            last_modified: row.get(2)?,
            price_change_number: row.get(3)?,
        })
    })?;

    Ok(row)
}
