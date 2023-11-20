<script lang="ts">
	import type { AuctionType } from '$lib/types';
	import { formatSeconds, formatNumber, getCommand, setClipboard } from '$lib/utils';
	import { onDestroy, onMount } from 'svelte';

	export let auction: AuctionType;
	// export let lastAuction: boolean;

	$: copied = false;

	function handleClick() {
		setClipboard(getCommand(auction.uuid));

		copied = true;
		setTimeout(() => {
			copied = false;
		}, 500);
	}
</script>

{#if !copied}
	<tr class="auction" on:click={handleClick}>
		<td> {auction.item_amount} </td>
		<td> {auction.item_name} </td>
		<td> {formatNumber(auction.price)}$ </td>
		<td> {formatNumber(auction.lowest_price)}$ </td>
		<td> {formatNumber(auction.profit)}$ </td>
		<td> {formatSeconds(auction.time_remaining.secs)} </td>
		<td> {auction.auctioneer} </td>
	</tr>
{:else}
	<div class="copied">
		<p>Copied!</p>
	</div>
{/if}

<style>
	.auction {
		text-align: left;
		padding-left: 20px;
		height: 40px;

		color: var(--subtle-text-color);
		border-bottom: 1px solid var(--subtle-color);
	}

	.auction td {
		padding-left: 20px;
	}

	.auction:hover {
		background-color: var(--subtle-color);
		cursor: pointer;
	}

	.copied {
		width: 100vw;
		height: 40px;

		background-color: var(--subtle-color);

		display: flex;
		align-items: center;
		justify-content: center;

		border-bottom: 1px solid var(--subtle-color);
	}
</style>
