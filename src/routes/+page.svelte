<script lang="ts">
	// Functions
	import { auctions, updateAuctionData, updateTimeRemaining } from '$lib/stores/Auctions';
	import { WebviewWindow } from '@tauri-apps/api/window';

	// Components
	import ActionButtons from '$components/ActionButtons.svelte';
	import Spinner from '$components/Spinner.svelte';
	import Auction from '$components/Auction.svelte';
	import Loadingbar from '$components/Loadingbar.svelte';
	import Column from '$components/Column.svelte';

	// Assets
	import settingsIcon from '$lib/icons/settings.svg';

	// Init

	function openSettingsWindow() {
		const webview = new WebviewWindow('Settings', {
			url: '/settings'
		});

		webview.once('tauri://error', function (e) {
			console.log(`Error creating settings window: ${e}`);
		});
	}

	$: loading = true;

	updateAuctionData(() => {
		loading = false;
	});
	setInterval(updateTimeRemaining, 1000);
</script>

<div data-tauri-drag-region class="topbar">
	<p class="window-title">
		<img class="settings-icon" on:click={openSettingsWindow} src={settingsIcon} />
		Quick Flip
	</p>
	<ActionButtons />
</div>

{#if !loading}
	<table class="auctions-container">
		<thead>
			<tr class="table-header">
				<Column title="#" />
				<Column title="Item" />
				<Column title="Price" />
				<Column title="Lowest" />
				<Column title="Profit" />
				<Column title="Time Left" />
				<Column title="Auctioneer" />
			</tr>
		</thead>

		<colgroup>
			<col class="amount-col" style="width: 5%" /> /* Amount */
			<col style="width: 20%" /> /* Item */
			<col style="width: 12%" /> /* Price */
			<col style="width: 12%" /> /* Lowest */
			<col style="width: 12%" /> /* Profit */
			<col style="width: 12%" /> /* Time Left */
			<col style="width: 15%" /> /* Auctioneer */
		</colgroup>

		{#each $auctions as auction, i}
			<Auction {auction} />
		{/each}
	</table>
{:else}
	<div class="spinner-container">
		<Spinner />
	</div>
{/if}

<Loadingbar />

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
		box-shadow: 0px 5px 5px 0px var(--shadow-color) 0 4px 30px rgba(0, 0, 0, 0.1);

		position: fixed;
	}

	.window-title {
		margin-left: 15px;
		font-weight: bold;

		display: flex;
		align-items: center;
	}

	.settings-icon {
		width: 22px;
		padding-right: 15px;
	}

	.settings-icon:hover {
		cursor: pointer;
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

		margin-top: calc(50px); /* 10px padding + 50px topbar height */

		height: auto;
		width: 100vw;
	}

	.table-header {
		height: 30px;
		font-weight: bold;
		border-bottom: 3px solid var(--subtle-color);
	}

	.amount-col {
		border-right: 1px solid var(--subtle-color);
	}
</style>
