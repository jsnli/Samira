use serde::{Deserialize, Serialize};
use steamworks::{AppId, Client, ClientManager};

#[derive(Serialize, Deserialize)]
pub struct Achievement {
    pub api_name: String,
    pub name: String,
    pub desc: String,
    pub status: bool,
}

use std::panic::{self, AssertUnwindSafe};
pub fn start_client(appid: u32) -> Result<Client<ClientManager>, String> {
    let result = panic::catch_unwind(AssertUnwindSafe(|| {
        let (client, _single) = Client::init_app(AppId(appid)).unwrap();
        let _user_stats = client.user_stats();
        let _ = _user_stats.request_current_stats();

        client
    }));

    match result {
        Ok(client) => Ok(client),
        Err(panic_error) => Err(format!("Panic occured: {:?}", panic_error)),
    }
}

pub fn load_achievements(client: Client<ClientManager>) -> Result<Vec<Achievement>, String> {
    let result = panic::catch_unwind(AssertUnwindSafe(|| {
        let id: String = client.user().steam_id().raw().to_string();
        println!("get info. id: {}", id);

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
            println!("{}", name);
        }

        AchievementList
    }));

    match result {
        Ok(list) => Ok(list),
        Err(panic_error) => Err(format!("Panic occured: {:?}", panic_error)),
    }
}

pub fn commit_achievement(client: Client<ClientManager>, name: String, unlocked: bool) {
    let user_stats = client.user_stats();
    let achievement = user_stats.achievement(&name);
    if unlocked {
        let _ = achievement.set();
    } else {
        let _ = achievement.clear();
    }
    let _ = user_stats.store_stats();
}

pub fn store_stats(client: Client<ClientManager>) {
    let user_stats = client.user_stats();
    let _ = user_stats.store_stats();
}

pub fn load_statistics(appid: u32) -> Vec<String> {
    println!("loading stats");
    let s: Vec<String> = Vec::new();
    s
}
