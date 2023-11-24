import type { AuctionType, ConfigType } from '$lib/types';
import { config } from '$stores/Config';
import { invoke } from '@tauri-apps/api/tauri';
import { get } from 'svelte/store';

// Gets the config file
export async function getConfig(): Promise<ConfigType> {
	return invoke('tauri_get_config');
}

// Updates the config file
export async function setConfig(configToSet: ConfigType) {
	config.set(configToSet);
	return invoke('tauri_set_config', { config: configToSet });
}

// Syncs the config file with the config store
export async function updateConfigFile() {
	let configToSet = get(config);
	await setConfig(configToSet);
}

export async function getAuctions(): Promise<AuctionType[]> {
	return invoke('tauri_get_auctions');
}
