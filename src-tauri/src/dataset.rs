use reqwest::blocking::Client;
use std::time::Duration;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Game {
    pub appid: u32,
    pub name: String,
}

pub fn fetch_games() -> Result<Vec<Game>, Box<dyn std::error::Error>> {
    let url = "https://raw.githubusercontent.com/jsnli/steamappidlist/master/data/games_appid.json";

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

use fuzzy_matcher::skim::SkimMatcherV2;
use fuzzy_matcher::FuzzyMatcher;

pub fn fuzzy_search(games: &[Game], query: &str, limit: usize) -> Vec<Game> {
    if query.trim().is_empty() {
        return vec![];
    }

    let matcher = SkimMatcherV2::default();
    let mut scored: Vec<_> = games
        .iter()
        .filter_map(|g| matcher.fuzzy_match(&g.name, query).map(|score| (score, g)))
        .collect();

    scored.sort_by(|a, b| b.0.cmp(&a.0));

    scored
        .into_iter()
        .take(limit)
        .map(|(_, g)| g.clone())
        .collect()

}

