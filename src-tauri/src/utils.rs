use std::time::UNIX_EPOCH;

use crate::auctions::items::AuctionItem;

pub async fn get_name(uuid: &str) -> String {
    let url = format!(
        "https://sessionserver.mojang.com/session/minecraft/profile/{}",
        uuid
    );

    let response = reqwest::get(url).await.unwrap().text().await.unwrap();

    let json: serde_json::Value = serde_json::from_str(&response).unwrap();
    let name = json["name"].as_str().unwrap();

    name.to_string()
}

pub fn get_epoch() -> u128 {
    let now = std::time::SystemTime::now();
    let epoch = now.duration_since(UNIX_EPOCH).expect("Time went backwards");

    epoch.as_millis()
}

pub fn get_auction_price(auction: &AuctionItem) -> u64 {
    let starting_price = auction.starting_bid;
    let current_price = auction.highest_bid_amount;

    if auction.bin {
        return starting_price;
    }
    if current_price == 0 { 
        // No bids have been placed
        return starting_price;
    } else {
        // Bids have been placed
        return current_price;
    }
}