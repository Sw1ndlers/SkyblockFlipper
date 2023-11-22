// Prevents additional console window on Windows in release
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod auctions {
    pub mod constants;
    pub mod fetcher;
    pub mod handler;
    pub mod items;
    pub mod names;
}
mod utils {
    pub mod config;
    pub mod frontend;
    pub mod functions;
}

use auctions::{
    fetcher::{get_auction_items, get_item_amounts, get_lowest_prices},
    handler::get_profit_items,
    items::{AuctionItem, ProfitItem},
};
use utils::{
    config::{get_config, set_config, ConfigStruct},
    frontend::set_spinner_text,
};

use lazy_static::lazy_static;
use reqwest::Client;
use std::{fs, path::PathBuf};
use tauri::{command, Manager, Runtime, Window};
use window_shadows::set_shadow;

// const DEBUG: bool = false;
// const MINIMUM_PROFIT: u64 = 80000;
// const MAXIMUM_TIME: u64 = 60 * 10; // 10 minutes

#[tauri::command]
async fn tauri_get_config() -> ConfigStruct {
    return get_config().unwrap();
}

#[tauri::command]
async fn tauri_set_config(config: ConfigStruct) -> Result<(), String> {
    // let config = match serde_json::from_str(&config) {
    //     Ok(config) => config,
    //     Err(_) => return Err("Invalid config".to_string()),
    // };
    set_config(config).unwrap();

    Ok(())
}

#[command]
async fn tauri_get_auctions<R: Runtime>(window: Window<R>) -> Vec<ProfitItem> {
    let client = Client::new();
    let auction_items: Vec<AuctionItem>;

    let config = get_config().unwrap();

    if config.debug {
        let cache_path = PathBuf::from("auctions.json");

        if cache_path.exists() == false {
            let data = get_auction_items(&client, &window).await.unwrap();
            let json = serde_json::to_string(&data).unwrap();

            fs::write(&cache_path, json).unwrap();
        }
        let data = fs::read_to_string(&cache_path).unwrap();
        auction_items = serde_json::from_str(&data).unwrap();
    } else {
        auction_items = get_auction_items(&client, &window).await.unwrap();
    }

    set_spinner_text(&window, "Finding profitable items...");

    let lowest_prices = get_lowest_prices(&auction_items);
    let item_amounts = get_item_amounts(&auction_items);
    let profit_items = get_profit_items(
        &auction_items,
        lowest_prices,
        item_amounts,
        config.maximum_time,
        config.minimum_profit,
    )
    .await;

    return profit_items;
}

fn main() {
    set_config(ConfigStruct { 
        debug: true,
        ..Default::default()
    }).unwrap();

    tauri::Builder::default()
        .setup(|app| {
            let window = app.get_window("main").unwrap();
            set_shadow(&window, true).expect("Unsupported platform!");
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![tauri_get_auctions])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
