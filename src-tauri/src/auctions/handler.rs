use crate::utils::{get_auction_price, get_epoch, get_name};

use super::items::{AuctionItem, ProfitItem};
use super::names::normalize_name;

use std::{collections::HashMap, time::Duration};

pub async fn get_profit_items(
    items: &Vec<AuctionItem>,
    lowest_prices: HashMap<String, u64>,
    maximum_time: u64,
    minimum_profit: u64,
) -> Vec<ProfitItem> {
    let mut profit_items: Vec<ProfitItem> = Vec::new();

    for item in items {
        if item.bin {
            continue;
        }

        let current_time = get_epoch() as u64;

        if item.end < current_time {
            continue;
        }

        let time_remaining = Duration::from_millis(item.end - current_time);
        let item_name = normalize_name(item.item_name.clone());
        let auction_price = get_auction_price(item);

        if time_remaining.as_secs() > maximum_time {
            continue;
        }

        let lowest_price = match lowest_prices.get(&item_name) {
            Some(price) => price,
            None => continue,
        };

        let profit: i64 = *lowest_price as i64 - auction_price as i64;

        if profit < 0 {
            continue;
        }

        let auctioneer_name = get_name(&item.auctioneer).await;

        // get increase from lowest price
        let profit_percent = ((profit as f64 / *lowest_price as f64) * 100.0).round();

        let profit_item = ProfitItem {
            auctioneer: auctioneer_name,
            time_remaining: time_remaining,
            item_name: item_name,
            price: auction_price,
            profit: profit as u64,
            profit_percent: profit_percent,
            lowest_price: *lowest_price,
            uuid: item.uuid.clone(),
        };

        if profit as u64 > minimum_profit {
            profit_items.push(profit_item);
        }
    }

    profit_items
}
