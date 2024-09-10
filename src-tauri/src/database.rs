use rusqlite::Connection;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct App {
    pub appid: i32,
    pub name: String,
    pub last_modified: i32,
    pub price_change_number: i32,
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
    let url = "https://raw.githubusercontent.com/jsnli/steamappidlist/master/data/games_appid.json";

    let res = reqwest::get(url).await?;

    let apps: Vec<App> = res.json().await?;

    Ok(apps)
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
    let mut stmt = db.prepare("
        SELECT * FROM apps WHERE name LIKE ?"
    )?;

    let search_pattern = format!("%{}%", name);
    let rows = stmt.query_map([search_pattern], |row| {
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




