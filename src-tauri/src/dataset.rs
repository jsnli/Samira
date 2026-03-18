use reqwest::blocking::Client;
use std::time::Duration;
use std::path::PathBuf;
use std::time::SystemTime;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Game {
    pub appid: u32,
    pub name: String,
}

pub fn fetch_games() -> Result<(Vec<Game>, String), Box<dyn std::error::Error>> {
    let cache_path = get_cache_path()?;

    if cache_is_fresh(&cache_path) {
        let data = std::fs::read_to_string(&cache_path)?;
        let games = serde_json::from_str(&data)?;
        return Ok((games, "Database loaded from cache.".to_string()));
    }

    match fetch_network() {
        Ok(games) => {
            let _ = write_cache(&cache_path, &games);
            Ok((games, "Database updated from network.".to_string()))
        }
        Err(e) => {
            if cache_path.exists() {
                let data = std::fs::read_to_string(&cache_path)?;
                let games = serde_json::from_str(&data)?;
                Ok((games, "Network unavailable, using cached database.".to_string()))
            } else {
                Err(e)
            }
        }
    }
}

fn get_cache_path() -> Result<PathBuf, Box<dyn std::error::Error>> {
    let home = std::env::var("HOME")?;
    let dir = PathBuf::from(home).join(".samira");
    std::fs::create_dir_all(&dir)?;
    Ok(dir.join("cache.json"))
}

fn cache_is_fresh(path: &PathBuf) -> bool {
    std::fs::metadata(path)
        .and_then(|m| m.modified())
        .map(|mtime| {
            SystemTime::now()
                .duration_since(mtime)
                .map(|age| age.as_secs() < 86400)
                .unwrap_or(false)
        })
        .unwrap_or(false)
}

fn fetch_network() -> Result<Vec<Game>, Box<dyn std::error::Error>> {
    let url = "https://raw.githubusercontent.com/jsnli/steamappidlist/master/data/games_appid.json";
    let client = Client::builder()
        .timeout(Duration::from_secs(15))
        .user_agent("steam-fetch/0.1 (+https://github.com/jsnli)")
        .build()?;
    let resp = client.get(url).send()?;
    if !resp.status().is_success() {
        return Err(format!("HTTP error: {}", resp.status()).into());
    }
    Ok(resp.json()?)
}

fn write_cache(path: &PathBuf, games: &[Game]) -> Result<(), Box<dyn std::error::Error>> {
    let json = serde_json::to_string(games)?;
    std::fs::write(path, json)?;
    Ok(())
}

use fuzzy_matcher::skim::SkimMatcherV2;
use fuzzy_matcher::FuzzyMatcher;

pub fn fuzzy_search(games: &[Game], query: &str, limit: usize) -> Vec<Game> {
    let query = query.trim().to_lowercase();
    if query.is_empty() {
        return vec![];
    }

    let matcher = SkimMatcherV2::default();

    let mut scored: Vec<(i64, &Game)> = games
        .iter()
        .filter_map(|g| {
            let name_score = matcher.fuzzy_match(&g.name.to_lowercase(), &query);
            let id_score = matcher.fuzzy_match(&g.appid.to_string(), &query);
            name_score.or(id_score).map(|score| (score, g))
        })
        .collect();


    scored.sort_by(|a, b| b.0.cmp(&a.0));

    scored
        .into_iter()
        .take(limit)
        .map(|(_, g)| g.clone())
        .collect()

}

