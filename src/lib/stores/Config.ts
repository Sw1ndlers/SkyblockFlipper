import type { ConfigType } from '$lib/types';
import { getConfig } from '$lib/utils';
import { writable, type Writable } from 'svelte/store';

let currentConfig = getConfig();

export const config: Writable<ConfigType> = writable(await currentConfig);
