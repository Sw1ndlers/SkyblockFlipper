<!-- <th class="amount-th" scope="col"> # </th>
<th scope="col"> Item </th>
<th scope="col"> Price </th>
<th scope="col"> Lowest </th>
<th scope="col"> Profit </th>
<th scope="col"> Time Left </th>
<th scope="col"> Auctioneer </th> -->

<script lang="ts">
	export let title: string;

	import arrowIcon from '$lib/icons/arrow.svg';
	import { updateAuctions, setCurrentSort } from '$stores/Auctions';
	import { sortFunctions } from '$lib/constants/sorting';
	import { sortAuctionsBy } from '$lib/utils/auctions';
	import type { SortType } from '$lib/types';

	$: hovered = false;
	$: higherToLower = false;

	function handleClick() {
		higherToLower = !higherToLower;

		let currentSort: SortType = {
			sortFunction: sortFunctions[title],
			higherToLower
		};

		setCurrentSort(currentSort);
		updateAuctions((auctions) => {
			return sortAuctionsBy(auctions, currentSort);
		});
	}
</script>

<th scope="col" class="table-header" class:padding-right={title == 'Auctioneer'}>
	<div
		on:click={handleClick}
		on:mouseenter={() => (hovered = true)}
		on:mouseleave={() => (hovered = false)}
	>
		{title}

		{#if title != '#'}
			<img
				class:hidden={!hovered}
				class:icon-flipped={!higherToLower}
				src={arrowIcon}
				class="arrow-icon"
			/>
		{/if}
	</div>
</th>

<style>
	.hidden {
		display: none;
	}

	.icon-flipped {
		transform: rotate(180deg);
	}

	.table-header {
		padding-top: 10px;
		height: 30px;
		padding-left: 20px;
	}

	.table-header:hover {
		cursor: pointer;
	}

	.table-header div {
		display: flex;
		align-items: center;
		justify-content: space-between;
	}

	.padding-right {
		padding-right: 15px;
	}
</style>
