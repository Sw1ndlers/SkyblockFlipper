import { invoke } from '@tauri-apps/api/tauri';
import { Rarity } from '$lib/constants';
import type { AuctionType, SortType, ConfigType } from '$lib/types';

import { config } from '$lib/stores/Config';

// Gets the config file
export async function getConfig(): Promise<ConfigType> {
	return await invoke('tauri_get_config');
}

// Updates the config file
export async function setConfig(configToSet: ConfigType) {
    config.set(configToSet);
	return invoke('tauri_set_config', { config: configToSet });
}

// Auctions

export async function getAuctions(): Promise<AuctionType[]> {
	return invoke('tauri_get_auctions');
}

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

// Formatting

export function formatSeconds(seconds: number): string {
	const minutes = Math.floor(seconds / 60);
	const remainingSeconds = seconds % 60;

	return `${minutes}m ${remainingSeconds}s`;
}

export function formatNumber(number: number): string {
	return number.toLocaleString();
}

export function setClipboard(text: string): void {
	navigator.clipboard.writeText(text);
}

// Misc

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
