import type { ConfigType } from '$lib/types';
import { getConfig } from '$utils/tauri';
import { writable, type Writable } from 'svelte/store';

let currentConfig = await getConfig();

export const config: Writable<ConfigType> = writable(currentConfig);

// let config: Writable<ConfigType>;

// export async function getConfigStore(): Promise<Writable<ConfigType>> {
//     if (config == undefined) {
//         config = writable(await getConfig());
//     }
//     return config;
// }

// config.subscribe((value) => {
//     setConfig(value);
// });
