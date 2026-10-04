use anyhow::Result;
use serde::Deserialize;

const CONFIG_PATH: &str = "timezones.json";

#[derive(Deserialize, Debug)]
pub struct TzConfig {
    pub name: String,
    pub code: String,
}

pub fn load_from_json() -> Result<Vec<TzConfig>> {
    let file = std::fs::File::open(CONFIG_PATH)?;
    let timezones: Vec<TzConfig> = serde_json::from_reader(file)?;
    Ok(timezones)
}
