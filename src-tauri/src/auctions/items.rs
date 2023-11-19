use std::time::Duration;

use serde::{Deserialize, Serialize};

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
    pub price: u64,
    pub profit: u64,
    pub profit_percent: f64,
    pub lowest_price: u64,
    pub uuid: String,
}