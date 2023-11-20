use crate::utils::get_epoch;

use serde::{Deserialize, Serialize};
use std::time::Duration;

#[derive(Serialize, Deserialize, Debug)]
pub struct Bid {
    amount: u64,
    auction_id: String,
    bidder: String,
    profile_id: String,
    timestamp: u64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AuctionItem {
    pub auctioneer: String,
    pub bids: Vec<Bid>,
    pub bin: bool,
    pub category: String,
    pub claimed: bool,
    pub claimed_bidders: Vec<String>,
    pub coop: Vec<String>,
    pub end: u64,
    pub extra: String,
    pub highest_bid_amount: u64,
    pub item_bytes: String,
    pub item_lore: String,
    pub item_name: String,
    pub item_uuid: Option<String>,
    pub last_updated: u64,
    pub profile_id: String,
    pub start: u64,
    pub starting_bid: u64,
    pub tier: String,
    pub uuid: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ProfitItem {
    pub auctioneer: String,
    pub time_remaining: Duration,
    pub item_name: String,
    pub item_amount: u64,
    pub price: u64,
    pub profit: i64,
    pub profit_percent: f64,
    pub lowest_price: u64,
    pub uuid: String,
    pub rarity: String,
}

impl AuctionItem {
    pub async fn to_profit_item(&self, item_name: String, lowest_price: u64, item_amount: u64) -> Option<ProfitItem> {
        let price = self.get_price();
        let profit: i64 = lowest_price as i64 - price as i64;

        if profit < 0 {
            return None;
        }

        let epoch_now = get_epoch() as u64;

        if self.end < epoch_now {
            return None;
        }

        // TODO: Fix this
        let profit_percent = (((lowest_price - price) / price) * 100) as f64;
        // let profit_percent = round_to_place(profit_percent, 1);
        
        let time_remaining = Duration::from_millis(self.end - epoch_now);

        Some(ProfitItem {
            auctioneer: "".to_string(), // Assigned later
            time_remaining,
            item_name,
            item_amount,
            price,
            profit,
            profit_percent,
            lowest_price,
            uuid: self.uuid.clone(),
            rarity: self.tier.clone(),
        })
    }

    pub fn get_price(&self) -> u64 {
        let starting_price = self.starting_bid;
        let current_price = self.highest_bid_amount;

        if self.bin {
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
}

// let profit_item = ProfitItem {
//     auctioneer: auctioneer_name,
//     time_remaining: time_remaining,
//     item_name: item_name,
//     price: auction_price,
//     profit: profit as u64,
//     profit_percent: profit_percent,
//     lowest_price: *lowest_price,
//     uuid: item.uuid.clone(),
// };