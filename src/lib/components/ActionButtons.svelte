<script lang="ts">
	import { appWindow } from '@tauri-apps/api/window';
	import { onMount } from 'svelte';

	import maximizeIcon from '$lib/icons/maximize.svg';
	import closeIcon from '$lib/icons/close.svg';
	import minimizeIcon from '$lib/icons/minimize.svg';

	let actionButtons: { [key: string]: HTMLDivElement | null } = {
		minimize: null,
		maximize: null,
		close: null
	};

	onMount(() => {
		actionButtons.minimize?.addEventListener('click', () => appWindow.minimize());
		actionButtons.maximize?.addEventListener('click', () => appWindow.toggleMaximize());
		actionButtons.close?.addEventListener('click', () => appWindow.close());
	});
</script>

<div class="actions-container">
	<div class="action-button" bind:this={actionButtons.minimize}>
		<img src={minimizeIcon} alt="Minimize" />
	</div>

	<div class="action-button" bind:this={actionButtons.maximize}>
		<img src={maximizeIcon} alt="Maximize" />
	</div>

	<div class="action-button" bind:this={actionButtons.close}>
		<img src={closeIcon} alt="Close" />
	</div>
</div>

<style>
	.actions-container {
		display: flex;
		flex-direction: row;

		align-items: center;
		margin-left: auto;

		height: 100%;
		margin-right: 10px;
	}

	.action-button {
		margin: 5px;

		width: 25px;
		height: 25px;

		display: flex;
		align-items: center;
		justify-content: center;
	}

	.action-button img {
		width: 50%;
		height: 50%;
	}

	.action-button:hover {
		background-color: var(--accent-color);
		cursor: pointer;
	}
</style>
