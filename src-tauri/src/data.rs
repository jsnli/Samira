use reqwest::blocking::Client;
use std::time::Duration;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
struct Game {
    appid: u32,
    name: String,
}

fn fetch_games(url: &str) -> Result<Vec<Game>, Box<dyn std::error::Error>> {
    let client = Client::builder()
        .timeout(Duration::from_secs(15))
        .user_agent("steam-fetch/0.1 (+https://github.com/jsnli)")
        .build()?;

    let resp = client.get(url).send()?;
    if !resp.status().is_success() {
        return Err(format!("HTTP error: {}", resp.status()).into());
    }

    let games: Vec<Game> = resp.json()?;

    Ok(games)
}

