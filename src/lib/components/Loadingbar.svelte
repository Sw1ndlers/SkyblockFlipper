<script lang="ts" context="module">
	declare const window: any;

	import { tweened } from 'svelte/motion';
	import { cubicOut } from 'svelte/easing';

	let loadingBar: HTMLDivElement;

	const progress = tweened(0, {
		duration: 400,
		easing: cubicOut
	});

	window.setLoadingbarProgress = function (percent: number) {
		progress.set(percent);
	};

	progress.subscribe((value) => {
        if (loadingBar == undefined) return;

		loadingBar.style.width = `${value}%`;
	});
</script>

<div class="loading-bar" bind:this={loadingBar} />

<style>
	.loading-bar {
		position: absolute;
		bottom: 0;

		width: 100%;
		height: 8px;

		background-color: var(--accent-color);
	}
</style>
