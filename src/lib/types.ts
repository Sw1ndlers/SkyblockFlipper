export type Instant = {
	nanos: number;
	secs: number;
};

export type AuctionType = {
	auctioneer: string;
	time_remaining: Instant;
	item_name: string;
	item_amount: number;
	price: number;
	lowest_price: number;
	profit: number;
	profit_percent: number;
	uuid: string;
	rarity: string;
};

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

export type SortType = {
	sortFunction: (a: AuctionType, b: AuctionType) => number;
	higherToLower: boolean;
};
