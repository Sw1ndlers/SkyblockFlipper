import type { AuctionType, SortType } from '$lib/types';
import { writable, type Writable, get } from 'svelte/store';
import { getAuctions, sortAuctionsBy } from '$lib/utils';
import { sortFunctions } from '$lib/constants';

const firstSort: SortType = {
	sortFunction: sortFunctions['Profit'],
	higherToLower: true
};

export const auctions: Writable<AuctionType[]> = writable([]);
export const currentSort: Writable<SortType> = writable(firstSort);

export const { subscribe: subscribeAuctions, set: setAuctions, update: updateAuctions } = auctions;
export const {
	subscribe: subscribeCurrentSort,
	set: setCurrentSort,
	update: updateCurrentSort
} = currentSort;

export async function updateAuctionData(delay: number = 1, callback: Function) {
	while (true) {
		let data = (await getAuctions()) as AuctionType[];
		let sortBy = get(currentSort);

		setAuctions(sortAuctionsBy(data, sortBy));

		window.eval(`setLoadingbarProgress(100)`); // Set loadingbar to 100% when done loading

		callback();
		await new Promise((resolve) => setTimeout(resolve, delay)); // Delay to prevent crashing
	}
}

export function updateTimeRemaining() {
	updateAuctions((auctions) => {
		auctions = auctions
			.map((auction) => {
				auction.time_remaining.secs -= 1;
				return auction;
			})
			.filter((auction) => auction.time_remaining.secs > 0);
		return auctions;
	});
}
