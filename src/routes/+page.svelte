<script lang="ts">
	import ActionButtons from '$components/ActionButtons.svelte';
    import Spinner from '$components/Spinner.svelte';

    import { getAuctions } from '$lib/utils';
	import type { AuctionType } from '$lib/types';
    import Auction  from '$components/Auction.svelte' ;
	import type { SvelteComponent } from 'svelte';

	let auctions: AuctionType[] = [];
    let spinner: SvelteComponent;
    $: loading = true;

	getAuctions().then((data) => {
		auctions = data as AuctionType[];
        loading = false;
	});

	// Update time remaining every second
	setInterval(() => {
		auctions = auctions
			.map((auction) => {
				auction.time_remaining.secs -= 1;
				return auction;
			})
			.filter((auction) => auction.time_remaining.secs > 0);
	}, 1000);

	// Update auctions every 5 minutes
	setInterval(() => {
        loading = true;
		getAuctions().then((data) => {
			auctions = data as AuctionType[];
            loading = false;
		});
	}, 60000 * 5);

</script>

<div data-tauri-drag-region class="topbar">
	<p class="window-title">Sw1ndlers Flipper</p>
	<ActionButtons />
</div>

{#if !loading}
    <table class="auctions-container">
        <thead>
            <tr class="table-header">
                <th scope="col"> Item </th>
                <th scope="col"> Price </th>
                <th scope="col"> Lowest </th>
                <th scope="col"> Profit </th>
                <th scope="col"> Time Left </th>
                <th scope="col"> Auctioneer </th>
            </tr>
        </thead>

        <colgroup>
            <col style="width: 20%" /> /* Item */
            <col style="width: 15%" /> /* Price */
            <col style="width: 15%" /> /* Profit */
            <col style="width: 15%" /> /* Lowest */
            <col style="width: 10%" /> /* Time Left */
            <col style="width: 25%" /> /* Auctioneer */
        </colgroup>

        {#each auctions as auction}
            <Auction auction={auction} />
        {/each}
    </table>
{:else}
    <div class="spinner-container">
        <Spinner bind:this={spinner} />
    </div>
{/if}



<style>
	.topbar {
		height: 50px;
		width: 100vw;

		user-select: none;

		display: flex;
		flex-direction: row;
		align-items: center;


        backdrop-filter: blur(8.6px);
        background-color: rgba(14, 17, 24, 0.8);
    
		border-bottom: 1px solid var(--subtle-color);
		box-shadow: 0px 5px 5px 0px var(--shadow-color)
                    0 4px 30px rgba(0, 0, 0, 0.1);

        position: fixed;
	}

	.window-title {
		margin-left: 20px;
		font-weight: bold;
	}

    .spinner-container {
        width: 100vw;
        height: calc(100vh - 51px);

        margin-top: 50px; /* 50px topbar height */

        display: flex;
        align-items: center;
        justify-content: center;
    }

	.auctions-container {
		table-layout: fixed;
		display: table;
		border-collapse: collapse;

		text-align: left;

		margin-top: calc(10px + 50px); /* 10px padding + 50px topbar height */
		height: auto;
		width: 100vw;
	}

	.table-header {
		height: 30px;
		font-weight: bold;
		border-bottom: 3px solid var(--subtle-color);
	}

	.table-header th {
		padding-left: 20px;
	}


</style>
