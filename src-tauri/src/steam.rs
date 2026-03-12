use crate::vdf::{self, VdfMap};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::env;
use std::panic::{self, AssertUnwindSafe};
use std::sync::{Arc, Mutex};

use steamworks::{Client, UserStatsReceived};

#[derive(Serialize, Deserialize)]
pub struct Achievement {
    pub api_name: String,
    pub name: String,
    pub desc: String,
    pub status: bool,
}

#[derive(Serialize, Deserialize)]
pub struct Stat {
    pub api_name: String,
    pub name: String,
    pub min: i32,
    pub max: i32,
    pub value: i32,
}

#[derive(Serialize, Deserialize)]
pub struct User {
    user_name: String,
    user_steam_id: u64,
}

impl Default for User {
    fn default() -> Self {
        User {
            user_name: "No user found.".to_string(),
            user_steam_id: 0,
        }
    }
}

pub fn start_client(appid: u32) -> Result<Client, String> {
    let result = panic::catch_unwind(AssertUnwindSafe(|| {
        let waiting = Arc::new(Mutex::new(true));
        let waiting_clone = Arc::clone(&waiting);

        let client = Client::init_app(appid).unwrap();
        let user_stats = client.user_stats();
        let steam_user_id: u64 = client.user().steam_id().raw();

        user_stats.request_user_stats(steam_user_id);
        client.register_callback(move |_data: UserStatsReceived| {
            let mut waiting = waiting_clone.lock().unwrap();
            *waiting = false;
            println!("User Stats Received.");
        });

        client.run_callbacks();

        // to-do: handle this more gracefully
        for _ in 0..10 {
            client.run_callbacks();
            ::std::thread::sleep(::std::time::Duration::from_millis(100));

            let waiting = waiting.lock().unwrap();
            if *waiting == false {
                break;
            }
        }

        client
    }));

    match result {
        Ok(client) => Ok(client),
        Err(panic_error) => Err(format!("Panic occured: {:?}", panic_error)),
    }
}

pub fn retrieve_user(client: Client) -> User {
    User {
        user_name: client.friends().name(),
        user_steam_id: client.user().steam_id().raw(),
    }
}

pub fn load_achievements(client: Client) -> Result<Vec<Achievement>, String> {
    let result = panic::catch_unwind(AssertUnwindSafe(|| {
        let user_stats = client.user_stats();

        let mut AchievementList: Vec<Achievement> = Vec::new();
        let names = user_stats
            .get_achievement_names()
            .expect("Failed to get names");
        for name in names {
            let achievement_helper = user_stats.achievement(&name);
            let a: Achievement = Achievement {
                api_name: name.clone(),
                name: achievement_helper
                    .get_achievement_display_attribute("name")
                    .unwrap()
                    .to_string(),
                desc: achievement_helper
                    .get_achievement_display_attribute("desc")
                    .unwrap()
                    .to_string(),
                status: achievement_helper.get().unwrap(),
            };
            AchievementList.push(a);
        }

        AchievementList
    }));

    match result {
        Ok(list) => Ok(list),
        Err(panic_error) => Err(format!("Panic occured: {:?}", panic_error)),
    }
}

pub fn load_achievement_icons(appid: u32) -> HashMap<String, String> {
    let mut paths: HashMap<String, String> = HashMap::new();

    let game_root = match load_schema(appid) {
        Ok(r) => r,
        Err(e) => { println!("{}", e); return paths; }
    };

    let stats_map = match game_root.get("stats").and_then(|v| v.as_map()) {
        Some(m) => m,
        None => return paths,
    };

    for entry in stats_map.values().filter_map(|v| v.as_map()) {
        if entry.get("type").and_then(|v| v.as_str()) != Some("ACHIEVEMENTS") {
            continue;
        }
        let bits = match entry.get("bits").and_then(|v| v.as_map()) {
            Some(b) => b,
            None => continue,
        };
        for bit in bits.values().filter_map(|v| v.as_map()) {
            let api_name = match bit.get("name").and_then(|v| v.as_str()) {
                Some(n) => n.to_string(),
                None => continue,
            };
            let display = match bit.get("display").and_then(|v| v.as_map()) {
                Some(d) => d,
                None => continue,
            };
            if let Some(icon) = display.get("icon").and_then(|v| v.as_str()) {
                paths.insert(api_name.clone(), format!(
                    "https://cdn.cloudflare.steamstatic.com/steamcommunity/public/images/apps/{}/{}",
                    appid, icon
                ));
            }
            if let Some(icon_gray) = display.get("icon_gray").and_then(|v| v.as_str()) {
                paths.insert(api_name + "-gray", format!(
                    "https://cdn.cloudflare.steamstatic.com/steamcommunity/public/images/apps/{}/{}",
                    appid, icon_gray
                ));
            }
        }
        break;
    }

    paths
}

pub fn commit_achievement(client: Client, name: String, unlocked: bool) {
    let user_stats = client.user_stats();
    let achievement = user_stats.achievement(&name);
    if unlocked {
        let _ = achievement.set();
    } else {
        let _ = achievement.clear();
    }
}

pub fn store_stats(client: Client) {
    let user_stats = client.user_stats();
    let _ = user_stats.store_stats();
}

pub fn load_schema(appid: u32) -> std::io::Result<VdfMap> {
    let home = env::var("HOME")
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::NotFound, e))?;
    let path = format!(
        "{}/.steam/steam/appcache/stats/UserGameStatsSchema_{}.bin",
        home, appid
    );
    let bytes = std::fs::read(&path)?;
    let mut parser = vdf::Parser::new(&bytes);
    let mut root = parser.parse_object()
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e.to_string()))?;
    let game_root = root.values_mut().next()
        .and_then(|v| if let vdf::VdfValue::Nested(m) = v { Some(std::mem::take(m)) } else { None })
        .ok_or_else(|| std::io::Error::new(std::io::ErrorKind::InvalidData, "no game root"))?;
    Ok(game_root)
}

pub fn load_statistics(client: Client, appid: u32) -> Vec<Stat> {
    let user_stats = client.user_stats();

    let game_root = match load_schema(appid) {
        Ok(r) => r,
        Err(e) => { println!("{}", e); return Vec::new(); }
    };

    let stats_map = match game_root.get("stats").and_then(|v| v.as_map()) {
        Some(m) => m,
        None => return Vec::new(),
    };

    stats_map.values()
        .filter_map(|v| v.as_map())
        .filter(|m| m.get("type").and_then(|v| v.as_str()) == Some("INT"))
        .map(|m| {
            let api_name = m.get("name").and_then(|v| v.as_str()).unwrap_or("").to_string();
            let display_name = m.get("display")
                .and_then(|v| v.as_map())
                .and_then(|d| d.get("name"))
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string();
            let max = m.get("max").and_then(|v| v.as_int()).unwrap_or(0);
            let value = user_stats.get_stat_i32(&api_name).unwrap_or(0);
            Stat { api_name, name: display_name, min: 0, max, value }
        })
        .collect()
}

pub fn commit_statistics(client: Client, name: String, value: i32) {
    let user_stats = client.user_stats();
    let _ = user_stats.set_stat_i32(&name, value);
}
