import type { AuctionType } from '$lib/types';

const characters: string[] = [];
for (let i = 32; i < 127; i++) {
	characters.push(String.fromCharCode(i));
}

// Sorting

function sortAlphabetically(a: string, b: string) {
	let a1 = characters.indexOf(a.charAt(0).toUpperCase());
	let b1 = characters.indexOf(b.charAt(0).toUpperCase());

	if (a1 < b1) return 1;
	if (a1 > b1) return -1;
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

// Rarity

export enum Rarity {
	Common,
	Uncommon,
	Rare,
	Epic,
	Legendary,
	Mythic,
	Special,
	VerySpecial,
	Supreme
}

type RarityColors = {
	[K in Rarity]: string;
};

export const rarityColors: RarityColors = {
	[Rarity.Common]: '#ffffff',
	[Rarity.Uncommon]: '#5555ff',
	[Rarity.Rare]: '#5555FF',
	[Rarity.Epic]: '#aa00aa',
	[Rarity.Legendary]: '#ffaa00',
	[Rarity.Mythic]: '#ff55ff',
	[Rarity.Special]: '#ff5555',
	[Rarity.VerySpecial]: '#ff5555',
	[Rarity.Supreme]: '#55ffff'
};
