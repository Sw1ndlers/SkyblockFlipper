use crate::utils::{get_epoch, get_name};

use super::items::{AuctionItem, ProfitItem};
use super::names::normalize_name;

use std::{collections::HashMap, time::Duration};

pub async fn get_profit_items(
    items: &Vec<AuctionItem>,
    lowest_prices: HashMap<String, u64>,
    item_amounts: HashMap<String, u64>,
    maximum_time: u64,
    minimum_profit: u64,
) -> Vec<ProfitItem> {
    let mut profit_items: Vec<ProfitItem> = Vec::new();

    for item in items {
        if item.bin {
            continue;
        }
    
        let item_name = normalize_name(item.item_name.clone());

        let lowest_price = match lowest_prices.get(&item_name) {
            Some(price) => price,
            None => continue,
        };
        let item_amount = match item_amounts.get(&item_name) {
            Some(amount) => amount,
            None => continue,
        };

        let profit_item = item.to_profit_item(item_name, *lowest_price, *item_amount).await;

        match profit_item {
            Some(mut profit_item) => {
                if profit_item.time_remaining.as_secs() > maximum_time {
                    continue;
                }
                if profit_item.profit < minimum_profit as i64 {
                    continue;
                }

                profit_item.auctioneer = get_name(&item.auctioneer).await;
                profit_items.push(profit_item);
            },
            None => continue,
        }
    }

    profit_items
}
