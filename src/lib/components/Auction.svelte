<script lang="ts">
	import type { AuctionType } from '$lib/types';
	import { rarityColors } from '$lib/constants';
	import {
		formatSeconds,
		formatNumber,
		getCommand,
		setClipboard,
		rarityFromString
	} from '$lib/utils';

	export let auction: AuctionType;

	$: color = rarityColors[rarityFromString(auction.rarity)];
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
		<td style="color: {color}"> {auction.item_name} </td>
		<td class="price"> {formatNumber(auction.price)}$ </td>
		<td class="lowest"> {formatNumber(auction.lowest_price)}$ </td>
		<td class="profit"> {formatNumber(auction.profit)}$ </td>
		<td class="time"> {formatSeconds(auction.time_remaining.secs)} </td>
		<td class="auctioneer"> {auction.auctioneer} </td>
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

		/* color: var(--subtle-text-color); */
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

	.profit {
		color: lightgreen;
	}

	.price,
	.lowest {
		color: lightskyblue;
	}

	.auctioneer {
		color: lightcoral;
	}

	.time {
		color: lightgoldenrodyellow;
	}
</style>
