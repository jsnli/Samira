use rusqlite::{params, Connection};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct App {
    pub appid: i32,
    pub name: String,
    pub last_modified: Option<i32>,
    pub price_change_number: Option<i32>,
}

#[derive(Serialize, Deserialize)]
pub struct AppList {
    apps: Vec<App>,
}

#[derive(Serialize, Deserialize)]
pub struct ApiResponse {
    applist: AppList
}

pub fn init_db() -> Result<Connection, rusqlite::Error> {
    let db = Connection::open_in_memory()?;

    db.execute(
        "CREATE TABLE IF NOT EXISTS apps (
            appid INTEGER PRIMARY KEY,
            name TEXT NOT NULL,
            last_modified INTEGER NOT NULL,
            price_change_number INTEGER NOT NULL
        )",
        (),
    )?;

    Ok(db)
}

pub async fn request_data() -> Result<Vec<App>, reqwest::Error> {
    let games_res = reqwest::get("https://raw.githubusercontent.com/jsnli/steamappidlist/master/data/games_appid.json").await?;
    let app_null_filter: Vec<Option<App>> = games_res.json().await?;

    let mut apps: Vec<App> = app_null_filter.into_iter()
        .filter_map(|app| app)
        .collect();

    let software_res = reqwest::get("https://raw.githubusercontent.com/jsnli/steamappidlist/master/data/software_appid.json").await?;
    let software: Vec<App> = software_res.json().await?;

    apps.extend(software);

    Ok(apps)
}

pub async fn request_app_name(appid: i32) -> Result<String, reqwest::Error> {
    let url = "https://api.steampowered.com/ISteamApps/GetAppList/v2/";

    let res: ApiResponse = reqwest::get(url).await?.json().await?;

    let mut name: String = String::new();

    match res.applist.apps.iter().find(|app| app.appid == appid) {
        Some(app) => name = String::from(app.name.clone()),
        None => println!("App not found"),
    }

    Ok(name) 
}

pub fn populate_data(db: &mut Connection, apps: Vec<App>) {
    let t = db.transaction().expect("test");
    {
        let mut insertion_sql = t.prepare("INSERT INTO apps (appid, name, last_modified, price_change_number) VALUES (?, ?, ?, ?)").expect("test");

        for app in apps {
            let _ = insertion_sql.execute((
                &app.appid,
                &app.name,
                &app.last_modified,
                &app.price_change_number,
            ));
        }
    }

    let _ = t.commit();
}

pub fn query_id(db: &mut Connection, appid: i32) -> Result<App, rusqlite::Error> {
    let mut stmt = db.prepare(
        "SELECT appid, name, last_modified, price_change_number FROM apps WHERE appid = ?",
    )?;

    let row: App = stmt.query_row(&[&appid], |row| {
        Ok(App {
            appid: row.get(0)?,
            name: row.get(1)?,
            last_modified: row.get(2)?,
            price_change_number: row.get(3)?,
        })
    })?;

    Ok(row)
}

pub fn query_name(db: &mut Connection, name: String) -> Result<Vec<App>, rusqlite::Error> {
    let mut stmt = db.prepare(
        "SELECT * FROM apps WHERE name LIKE ? or appid LIKE ?",
    )?;

    let search_pattern = format!("%{}%", name);
    let rows = stmt.query_map(params![search_pattern, search_pattern], |row| {
        Ok(App {
            appid: row.get(0)?,
            name: row.get(1)?,
            last_modified: row.get(2)?,
            price_change_number: row.get(3)?,
        })
    })?;

    let result: Vec<App> = rows.collect::<Result<Vec<App>, rusqlite::Error>>()?;
    Ok(result)
}
