use steamworks::{Client,AppId, ClientManager};

pub struct Achievement {
    pub name: String,
    pub desc: String,
    pub status: bool
}

pub fn start(appid: u32) -> Client<ClientManager> {
    let (client, _single) = Client::init_app(AppId(1966900)).unwrap();
    let _user_stats = client.user_stats();

    client

    // let _cb = client.register_callback(|p: PersonaStateChange| {
    //     println!("Got callback: {:?}", p);
    // });
    //
    // match user_stats.get_achievement_names() {
    //     Some(names) => {
    //         for name in names {
    //             println!("{}", name);
    //         }
    //     }
    //     None => {
    //         eprintln!("No achievement names found");
    //     }
    // }
    //
    // for _ in 0..50 {
    //     single.run_callbacks();
    //     ::std::thread::sleep(::std::time::Duration::from_millis(100));
    // }
}

pub fn load_achievements(client: Client<ClientManager>) {
    let id: String = client.user().steam_id().raw().to_string();
    println!("get info. id: {}", id);

    let user_stats = client.user_stats();

    // match user_stats.get_achievement_names() {
    //     Some(names) => {
    //         for name in names {
    //             println!("{}", name);
    //         }
    //     }
    //     None => {
    //         eprintln!("No achievement names found");
    //     }
    // }

    let mut AchievementList: Vec<Achievement> = Vec::new();
    let names = user_stats.get_achievement_names().expect("Failed to get names");
    for name in names {
        let achievement_helper = user_stats.achievement(&name);
        let a: Achievement = Achievement {
            name: achievement_helper.get_achievement_display_attribute("name").unwrap().to_string(),
            desc: achievement_helper.get_achievement_display_attribute("desc").unwrap().to_string(),
            status: achievement_helper.get().unwrap(),
        };
        AchievementList.push(a);
        println!("{}", name);

    }

    let x = user_stats.achievement("ACH_CROSSBOW");

    println!("{}", x.get_achievement_display_attribute("name").unwrap());
    println!("{}", x.get_achievement_display_attribute("desc").unwrap());
    println!("{}", x.get_achievement_display_attribute("hidden").unwrap());
    println!("{}", x.get().unwrap());
}
