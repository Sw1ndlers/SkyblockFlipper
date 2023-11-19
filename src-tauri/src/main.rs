// Prevents additional console window on Windows in release
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod auctions {
    pub mod constants;
    pub mod fetcher;
    pub mod handler;
    pub mod items;
    pub mod names;
}
mod utils;



use auctions::{fetcher::get_lowest_prices, items::{ProfitItem}};
use reqwest::Client;

use crate::auctions::{fetcher::get_auction_items, handler::get_profit_items};

const MINIMUM_PROFIT: u64 = 100000;
const MAXIMUM_TIME: u64 = 60 * 5; // 5 minutes


#[tauri::command]
async fn tauri_get_auctions() -> Vec<ProfitItem> {
    let client = Client::new();

    let auction_items = get_auction_items(&client).await.unwrap();
    // fs::write("auction_items.json", serde_json::to_string_pretty(&auction_items).unwrap()).unwrap();

    // let auction_items = fs::read_to_string("auction_items.json").unwrap();
    // let auction_items: Vec<AuctionItem> = serde_json::from_str(&auction_items).unwrap();

    let lowest_prices = get_lowest_prices(&auction_items);
    let profit_items =
        get_profit_items(&auction_items, lowest_prices, MAXIMUM_TIME, MINIMUM_PROFIT).await;

    return profit_items;
}

use tauri::Manager;
use window_shadows::set_shadow;

fn main() {
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
