<script lang="ts">
	import Setting from '$components/Setting.svelte';
	import { config } from '$stores/Config';
	import { get } from 'svelte/store';

	// function updateMinimumProfit(val: number) {
	//     config.update((c) => {
	//         c["minimum_profit"] = val;
	//         return c;
	//     });
	// }
	function updateMinimumProfit(val: number) {
		let currentConfig = get(config);

		currentConfig['minimum_profit'] = val;
		config.set(currentConfig);
	}

	function updateMaximumTime(val: number) {
		let currentConfig = get(config);

		currentConfig['maximum_time'] = val;
		config.set(currentConfig);
	}

	function updateRefreshDelay(val: number) {
		let currentConfig = get(config);

		currentConfig['refresh_delay'] = val;
		config.set(currentConfig);
	}

	const currentConfig = get(config);

	let settingsList = [
		{
			title: 'Minimum Profit',
			caption: 'Sets your minimum amount of profit',
			unit: 'coins',
			currentValue: currentConfig['minimum_profit'],
			callback: updateMinimumProfit
		},
		{
			title: 'Maximum Time',
			caption: 'Sets the maximum time an auction can have',
			unit: 'minutes',
			currentValue: currentConfig['maximum_time'],
			callback: updateMaximumTime
		},
		{
			title: 'Refresh Delay',
			caption: 'Sets the delay between each refresh',
			unit: 'seconds',
			currentValue: currentConfig['refresh_delay'],
			callback: updateRefreshDelay
		}
	];
</script>

{#each settingsList as setting}
	<Setting {...setting} />
{/each}
