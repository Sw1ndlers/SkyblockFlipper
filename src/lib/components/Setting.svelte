<script lang="ts">
	import { onMount } from 'svelte';
	import { updateConfigFile } from '$utils/tauri';

	let inputElement: HTMLInputElement;

	export let callback: (value: number) => void;
	export let title: string;
	export let caption: string;
	export let unit: string;
	export let currentValue: number;

	onMount(() => {
		// Lose focus on enter
		inputElement.addEventListener('keydown', (e) => {
			if (e.key === 'Enter') {
				inputElement.blur();
			}
		});

		// Detect defocus
		inputElement.addEventListener('blur', () => {
			callback(Number(inputElement.value));
			updateConfigFile();
		});
	});
</script>

<div class="setting-container">
	<div class="info">
		<p class="title">{title}</p>
		<p class="caption">{caption}</p>
	</div>

	<div class="input-container">
		<input class="input" type="number" value={currentValue} bind:this={inputElement} />
		<p class="unit">{unit}</p>
	</div>
</div>

<style>
	.setting-container {
		padding-top: 10px;
		padding-bottom: 10px;

		width: 100vw;
		height: 60px;
		display: flex;
		justify-content: space-between;
		align-items: center;

		border-bottom: 1px solid var(--subtle-color);
	}

	.info {
		margin-left: 20px;
	}

	.title {
		font-weight: bold;
	}

	.caption {
		color: var(--subtle-text-color);
		font-size: 15px;
	}

	.input-container {
		display: flex;
		width: 240px;
		margin-right: 20px;
	}

	.input {
		margin-right: 10px;
		color: var(--subtle-text-color);
		background-color: transparent;
		border: 1px solid var(--subtle-color);
		outline-color: var(--subtle-color);
	}

	.unit {
		color: var(--subtle-text-color);
	}

	/* Disable Up and Down Arrows */
	.input::-webkit-outer-spin-button,
	.input::-webkit-inner-spin-button {
		-webkit-appearance: none;
		margin: 0;
	}
</style>
