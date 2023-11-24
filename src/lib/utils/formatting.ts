import { Rarity } from '$lib/types';

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
