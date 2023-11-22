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
    pub mod frontend;
    pub mod functions;
}

use auctions::{
    fetcher::{get_item_amounts, get_lowest_prices, get_auction_items},
    handler::get_profit_items,
    items::{AuctionItem, ProfitItem},
};
use utils::frontend::set_spinner_text;

use std::{fs, path::PathBuf};
use reqwest::Client;

const DEBUG: bool = false;
const MINIMUM_PROFIT: u64 = 80000;
const MAXIMUM_TIME: u64 = 60 * 10; // 10 minutes

#[command]
async fn tauri_get_auctions<R: Runtime>(window: Window<R>) -> Vec<ProfitItem> {
    let client = Client::new();

    let auction_items: Vec<AuctionItem>;

    if DEBUG {
        if PathBuf::from("auctions.json").exists() == false {
            let data = get_auction_items(&client, &window).await.unwrap();
            fs::write("auctions.json", serde_json::to_string(&data).unwrap()).unwrap();
        }
        auction_items = serde_json::from_str(&fs::read_to_string("auctions.json").unwrap()).unwrap();
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
        MAXIMUM_TIME,
        MINIMUM_PROFIT,
    )
    .await;

    return profit_items;
}

use tauri::{command, Manager, Runtime, Window};
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
