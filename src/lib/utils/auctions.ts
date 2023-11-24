import { Rarity, type AuctionType, type SortType } from '$lib/types';

export function sortAuctionsBy(auctions: AuctionType[], sortBy: SortType) {
	let sorted: AuctionType[] = [];

	if (sortBy.sortFunction != undefined) {
		new Error('Sort function is undefined');
	}

	sorted = auctions.sort(sortBy.sortFunction!);

	if (sortBy.higherToLower) {
		sorted = sorted.reverse();
	}

	return sorted;
}

export function getCommand(uuid: string): string {
	// 885184dc189b421c80acc51ed8eef34a -> 885184dc-189b-421c-80ac-c51ed8eef34a
	uuid =
		uuid.slice(0, 8) +
		'-' +
		uuid.slice(8, 12) +
		'-' +
		uuid.slice(12, 16) +
		'-' +
		uuid.slice(16, 20) +
		'-' +
		uuid.slice(20);
	return `/viewauction ${uuid}`;
}

export function rarityFromString(rarity: string): Rarity {
	switch (rarity) {
		case 'COMMON':
			return Rarity.Common;
		case 'UNCOMMON':
			return Rarity.Uncommon;
		case 'RARE':
			return Rarity.Rare;
		case 'EPIC':
			return Rarity.Epic;
		case 'LEGENDARY':
			return Rarity.Legendary;
		case 'MYTHIC':
			return Rarity.Mythic;
		case 'SPECIAL':
			return Rarity.Special;
		case 'VERY_SPECIAL':
			return Rarity.VerySpecial;
		case 'SUPREME':
			return Rarity.Supreme;
		default:
			return Rarity.Common;
	}
}
