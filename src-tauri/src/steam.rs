use regex::Regex;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::Read;
use std::panic::{self, AssertUnwindSafe};
use std::sync::{Arc, Mutex};

use steamworks::{Client, ClientManager, UserStatsReceived};

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

pub fn start_client(appid: u32) -> Result<Client<ClientManager>, String> {
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
        for _ in 0..50 {
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

pub fn retrieve_user(client: Client<ClientManager>) -> User {
    User {
        user_name: client.friends().name(),
        user_steam_id: client.user().steam_id().raw(),
    }
}

pub fn load_achievements(client: Client<ClientManager>) -> Result<Vec<Achievement>, String> {
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
    paths.insert(String::from("Blue String"), String::from("10"));

    let re = Regex::new(r"name(.*?)displaynameenglish(.*?)(icon_gray|icon)(.*?).jpg").unwrap();

    match load_schema(appid) {
        Ok(data) => {
            let captures: Vec<(String, String)> = re
                .captures_iter(&data)
                .map(|caps| {
                    let name: String = caps[1].to_string();
                    let hash: String = caps[4].to_string();

                    (name, hash)
                })
                .collect();
            
            for (key, value) in captures {
                paths.insert(key, format!("https://cdn.cloudflare.steamstatic.com/steamcommunity/public/images/apps/{}/{}", appid, value));
            }
        }
        Err(e) => {
            println!("{}", e);
        }
    }
    paths 
}

pub fn commit_achievement(client: Client<ClientManager>, name: String, unlocked: bool) {
    let user_stats = client.user_stats();
    let achievement = user_stats.achievement(&name);
    if unlocked {
        let _ = achievement.set();
    } else {
        let _ = achievement.clear();
    }
}

pub fn store_stats(client: Client<ClientManager>) {
    let user_stats = client.user_stats();
    let _ = user_stats.store_stats();
}

pub fn load_schema(appid: u32) -> std::io::Result<String> {
    let name = env::var("USER").unwrap_or("root".to_string());
    let path = format!(
        "/home/{}/.steam/steam/appcache/stats/UserGameStatsSchema_{}.bin",
        name, appid
    );

    let mut file = File::open(&path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;
    let content = String::from_utf8_lossy(&buffer);
    let cleaned: String = content
        .chars()
        .filter(|&c| c.is_ascii_graphic() || c.is_ascii_whitespace())
        .collect();

    Ok(cleaned)
}

pub fn load_statistics(client: Client<ClientManager>, appid: u32) -> Vec<Stat> {
    let user_stats = client.user_stats();

    let re = Regex::new(r"type1name(.*?)displayname(.*?)(?:maxchange(\d+))?(?:incrementonly(\d+))?(?:min(\d+)max(\d+)|max(\d+)min(\d+))Default").unwrap();

    let mut stats: Vec<Stat> = Vec::new();

    match load_schema(appid) {
        Ok(data) => {
            stats = re
                .captures_iter(&data)
                .map(|caps| {
                    let api_name = caps
                        .get(1)
                        .map_or(String::new(), |f| f.as_str().to_string());

                    let name = caps
                        .get(2)
                        .map_or(String::new(), |f| f.as_str().to_string());
                    let value = user_stats.get_stat_i32(&api_name).unwrap_or(0);

                    let min_val_one = caps
                        .get(5)
                        .map_or(String::new(), |f| f.as_str().to_string());
                    let min_val_two = caps
                        .get(7)
                        .map_or(String::new(), |f| f.as_str().to_string());
                    let max_val_one = caps
                        .get(6)
                        .map_or(String::new(), |f| f.as_str().to_string());
                    let max_val_two = caps
                        .get(8)
                        .map_or(String::new(), |f| f.as_str().to_string());

                    let mut min = if min_val_one.len() > min_val_two.len() {
                        &min_val_one
                    } else {
                        &min_val_two
                    };
                    let mut max = if max_val_one.len() > max_val_two.len() {
                        &max_val_one
                    } else {
                        &max_val_two
                    };

                    if min > max {
                        let temp = min;
                        min = max;
                        max = temp;
                    }

                    Stat {
                        api_name,
                        name,
                        min: min.parse::<i32>().unwrap_or(0),
                        max: max.parse::<i32>().unwrap_or(0),
                        value,
                    }
                })
                .collect();
        }
        Err(e) => {
            println!("{}", e);
        }
    }
    stats
}

pub fn commit_statistics(client: Client<ClientManager>, name: String, value: i32) {
    let user_stats = client.user_stats();
    let _ = user_stats.set_stat_i32(&name, value);
}
