import { invoke } from '@tauri-apps/api/tauri';
import type { AuctionType, SortType } from '$lib/types';

export function sortAuctionsBy(auctions: AuctionType[], sortBy: SortType) {
    let sorted: AuctionType[] = [];

    if (sortBy.sortFunction != undefined) {
        sorted = auctions.sort(sortBy.sortFunction!);
    } else {
        sorted = auctions.sort(); // Sort alphabetically
    }
    
    if (sortBy.higherToLower) {
        sorted = sorted.reverse();
    }

    return sorted;
}

export function formatSeconds(seconds: number): string {
    const minutes = Math.floor(seconds / 60);
    const remainingSeconds = seconds % 60;

    return `${minutes}m ${remainingSeconds}s`
}

export function formatNumber(number: number): string {
    return number.toLocaleString();
}

export function getCommand(uuid: string): string {
    // 885184dc189b421c80acc51ed8eef34a -> 885184dc-189b-421c-80ac-c51ed8eef34a
    uuid = uuid.slice(0, 8) + "-" + uuid.slice(8, 12) + "-" + uuid.slice(12, 16) + "-" + uuid.slice(16, 20) + "-" + uuid.slice(20);
    return `/viewauction ${uuid}`;
}

export function setClipboard(text: string): void {
    navigator.clipboard.writeText(text);
}

export async function getAuctions() {
    return await invoke('tauri_get_auctions');
}