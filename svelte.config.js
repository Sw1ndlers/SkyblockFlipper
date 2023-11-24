import adapter from '@sveltejs/adapter-static'; // This was changed from adapter-auto
import { vitePreprocess } from '@sveltejs/kit/vite';

/** @type {import('@sveltejs/kit').Config} */
const config = {
	// Consult https://kit.svelte.dev/docs/integrations#preprocessors
	// for more information about preprocessors
	preprocess: vitePreprocess(),
	onwarn: (warning, handler) => {
		if (warning.code.startsWith('a11y-')) {
			return;
		}
		handler(warning);
	},
	kit: {
		adapter: adapter(),
		prerender: {
			handleHttpError: 'warn'
		},
		alias: {
			$components: './src/lib/components',
			$styles: './src/lib/styles',
			$icons: './src/lib/icons',
			$classes: './src/lib/classes',
            $stores: './src/lib/stores',
            $utils: './src/lib/utils',
		}
	},

	onwarn: (warning, handler) => {
		if (warning.code.startsWith('a11y-')) return;
		handler(warning);
	}
};

export default config;
