use steamworks::{AppId, Client, ClientManager};

pub fn start(appid: u32) -> Client<ClientManager> {
    
        let (client, _single) = Client::init_app(AppId(appid)).unwrap();
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

pub fn get_info(client: Client<ClientManager>) {
    let id: String = client.user().steam_id().raw().to_string();

    println!("get info. id: {}", id);

}
