import type { AuctionType } from '$lib/types';

function sortAlphabetically(a: string, b: string) {
	if (a < b) return -1;
	if (a > b) return 1;
	return 0;
}

export const sortFunctions: { [key: string]: (a: AuctionType, b: AuctionType) => number } = {
	Item: (a: AuctionType, b: AuctionType) => {
		return sortAlphabetically(a.item_name, b.item_name);
	},
	Auctioneer: (a: AuctionType, b: AuctionType) => {
		return sortAlphabetically(a.auctioneer, b.auctioneer);
	},
	Price: (a: AuctionType, b: AuctionType) => {
		return a.price - b.price;
	},
	Lowest: (a: AuctionType, b: AuctionType) => {
		return a.lowest_price - b.lowest_price;
	},
	Profit: (a: AuctionType, b: AuctionType) => {
		return a.profit - b.profit;
	},
	'Time Left': (a: AuctionType, b: AuctionType) => {
		return a.time_remaining.secs - b.time_remaining.secs;
	},
	'#': (a: AuctionType, b: AuctionType) => {
		return a.item_amount - b.item_amount;
	}
};
