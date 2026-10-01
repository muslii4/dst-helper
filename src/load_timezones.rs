use anyhow::Result;
use serde::Deserialize;

const DEFAULT_CONFIG_PATH: &str = "timezones.json";

#[derive(Deserialize, Debug)]
pub struct TzConfig {
    pub name: String,
    pub code: String,
}

pub fn load_from_json(path: Option<&str>) -> Result<Vec<TzConfig>> {
    let path = match path {
        None => DEFAULT_CONFIG_PATH,
        Some(p) => p,
    };

    let file = std::fs::File::open(path)?;
    let timezones: Vec<TzConfig> = serde_json::from_reader(file)?;
    Ok(timezones)
}