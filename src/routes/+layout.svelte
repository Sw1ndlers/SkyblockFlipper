<svelte:head>
	<link rel="preconnect" href="https://fonts.googleapis.com" />
	<link rel="preconnect" href="https://fonts.gstatic.com" crossorigin="true" />
	stylesheet" />
</svelte:head>

<slot />

<style>
	:root {
		--main-background-color: #0e1118;
		--secondary-background-color: #1a1f28;

		--topbar-color: var(--main-background-color);
		--accent-color: #48546a;

		--shadow-color: #0d0f13;

		--font-color: #ffffff;
		--subtle-color: #202632;
		--subtle-text-color: #c2c7d0;

		background-color: var(--main-background-color);
		color: var(--font-color);
		overflow-x: hidden;

		font-family: sans-serif;

		display: flex;
		align-items: center;
	}
</style>
