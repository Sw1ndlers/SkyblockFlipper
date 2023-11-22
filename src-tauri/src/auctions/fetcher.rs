use super::{items::AuctionItem, names::normalize_name};
use crate::utils::frontend::{set_loadingbar_progress, set_spinner_text};

use reqwest::Client;
use serde_json::Value;
use std::collections::HashMap;
use tauri::{Runtime, Window};

async fn get_auction_urls(client: &Client) -> Result<Vec<String>, anyhow::Error> {
    let auction_data: serde_json::Value = serde_json::from_str(
        client
            .get("https://api.hypixel.net/v2/skyblock/auctions")
            .send()
            .await?
            .text()
            .await?
            .as_str(),
    )
    .unwrap();

    let total_pages = auction_data["totalPages"].as_u64().unwrap();

    let urls: Vec<String> = (0..total_pages)
        .map(|page| format!("https://api.hypixel.net/v2/skyblock/auctions?page={}", page))
        .collect();

    Ok(urls)
}

pub async fn get_auction_items<R: Runtime>(
    client: &Client,
    window: &Window<R>,
) -> Result<Vec<AuctionItem>, anyhow::Error> {
    let urls = get_auction_urls(client).await?;
    let total_pages = urls.len();

    println!("Fetching {} pages", total_pages);

    let mut auctions: Vec<AuctionItem> = Vec::new();

    // Fetch each page
    for (i, url) in urls.iter().enumerate() {
        let fetching_text = format!("Fetching page {}/{}", i + 1, total_pages);

        println!("{fetching_text}");

        set_spinner_text(window, &fetching_text);
        set_loadingbar_progress(window, i as u64, total_pages as u64);

        // Get the auctions on this page
        let response: Value = serde_json::from_str(
            client
                .get(url.as_str())
                .send()
                .await?
                .text()
                .await?
                .as_str(),
        )
        .unwrap();

        let page_auctions = response["auctions"].as_array().unwrap().to_vec();

        for auction in page_auctions {
            // Convert the auction to a struct
            let mut auction: AuctionItem = serde_json::from_value(auction).unwrap();
            auction.item_name = normalize_name(auction.item_name);

            auctions.push(auction);
        }
    }

    println!("Fetched {} auctions", auctions.len());

    Ok(auctions)
}

pub fn get_item_amounts(auctions: &Vec<AuctionItem>) -> HashMap<String, u64> {
    let mut item_amounts: HashMap<String, u64> = HashMap::new();

    for auction in auctions {
        let item_name = auction.item_name.clone();

        if item_amounts.contains_key(&item_name) {
            let current_amount = item_amounts.get(&item_name).unwrap();
            item_amounts.insert(item_name, current_amount + 1);
        } else {
            item_amounts.insert(item_name, 1);
        }
    }

    item_amounts
}

pub fn get_lowest_prices(auctions: &Vec<AuctionItem>) -> HashMap<String, u64> {
    let mut lowest_prices: HashMap<String, u64> = HashMap::new();

    for auction in auctions {
        if auction.bin == false {
            continue;
        }

        let item_name = auction.item_name.clone();
        let auction_price = auction.get_price();

        if lowest_prices.contains_key(&item_name) {
            let current_lowest_price = lowest_prices.get(&item_name).unwrap();

            if auction_price < *current_lowest_price {
                lowest_prices.insert(item_name, auction_price);
            }
        } else {
            lowest_prices.insert(item_name, auction_price);
        }
    }

    lowest_prices
}
