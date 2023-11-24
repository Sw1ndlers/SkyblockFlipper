use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Serialize, Deserialize, Debug)]
pub struct ConfigStruct {
    pub debug: bool,         // Use to cache auction data
    pub minimum_profit: u64, // In Coins
    pub maximum_time: u64,   // In Minutes
    pub refresh_delay: u64,  // In Seconds
}

impl Default for ConfigStruct {
    fn default() -> Self {
        Self {
            debug: false,
            minimum_profit: 80000,
            maximum_time: 10,
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

    // let config: ConfigStruct = serde_json::from_str(&fs::read_to_string("config.json")?)?;

    let config: ConfigStruct = match serde_json::from_str(&fs::read_to_string("config.json")?) {
        Ok(config) => config,
        Err(_) => {
            print!("Invalid config file, creating new one...");
            create_config_file()?;
            
            serde_json::from_str(&fs::read_to_string("config.json")?)?
        }
    };

    return Ok(config);
}

pub fn set_config(config: ConfigStruct) -> anyhow::Result<()> {
    // let mut config = config;
    
    // // Convert maximum_time from minutes to milliseconds
    // config.maximum_time *= 60 * 1000;

    // // Convert refresh_delay from seconds to milliseconds
    // config.refresh_delay *= 1000;

    println!("{:?}", config);

    fs::write("config.json", serde_json::to_string_pretty(&config)?)?;

    Ok(())
}
