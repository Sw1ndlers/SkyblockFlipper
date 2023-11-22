use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Serialize, Deserialize, Debug)]
pub struct ConfigStruct {
    pub debug: bool,         // Use to cache auction data
    pub minimum_profit: u64, // In Coins
    pub maximum_time: u64,   // In Milliseconds
    pub refresh_delay: u64,  // In Milliseconds
}

impl Default for ConfigStruct {
    fn default() -> Self {
        Self {
            debug: false,
            minimum_profit: 80000,
            maximum_time: 60 * 10,
            refresh_delay: 0,
        }
    }
}

fn config_file_exists() -> bool {
    return fs::metadata("config.json").is_ok();
}

fn create_config_file() -> anyhow::Result<()> {
    let config = ConfigStruct::default();
    fs::write("config.json", serde_json::to_string(&config)?)?;

    Ok(())
}

pub fn get_config() -> anyhow::Result<ConfigStruct> {
    if config_file_exists() == false {
        create_config_file()?;
    }

    let config: ConfigStruct = serde_json::from_str(&fs::read_to_string("config.json")?)?;

    return Ok(config);
}

pub fn set_config(config: ConfigStruct) -> anyhow::Result<()> {
    fs::write("config.json", serde_json::to_string(&config)?)?;

    Ok(())
}
