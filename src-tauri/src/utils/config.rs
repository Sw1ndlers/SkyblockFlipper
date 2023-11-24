use platform_dirs::{AppDirs, UserDirs};
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

const CONFIG_FILE: &str = "config.json";
const CONFIG_DIR: &str = "QuickFlip";

fn config_file_exists() -> bool {
    let config_dir = AppDirs::new(Some(CONFIG_DIR), false).unwrap().data_dir;
    let config_file = config_dir.join(CONFIG_FILE);

    return config_file.exists();
}

fn create_config_file() -> anyhow::Result<()> {
    let default_config = ConfigStruct::default();
    let config_dir = AppDirs::new(Some(CONFIG_DIR), false).unwrap().data_dir;

    fs::create_dir_all(&config_dir)?;
    fs::write(&config_dir.join(CONFIG_FILE), serde_json::to_string_pretty(&default_config)?)?;

    Ok(())
}

pub fn get_config() -> anyhow::Result<ConfigStruct> {
    if config_file_exists() == false {
        create_config_file()?;
    }

    let config_dir = AppDirs::new(Some(CONFIG_DIR), false).unwrap().data_dir;
    let config_file = config_dir.join(CONFIG_FILE);

    let config: ConfigStruct = match serde_json::from_str(&fs::read_to_string(&config_file)?) {
        Ok(config) => config,
        Err(_) => {
            print!("Invalid config file, creating new one...");
            create_config_file()?;
            
            serde_json::from_str(&fs::read_to_string(&config_file)?)?
        }
    };

    Ok(config)
}

pub fn set_config(config: ConfigStruct) -> anyhow::Result<()> {
    // fs::write("config.json", serde_json::to_string_pretty(&config)?)?;
    let config_dir = AppDirs::new(Some(CONFIG_DIR), false).unwrap().data_dir;
    let config_file = config_dir.join(CONFIG_FILE);

    fs::write(&config_file, serde_json::to_string_pretty(&config)?)?;

    Ok(())
}
