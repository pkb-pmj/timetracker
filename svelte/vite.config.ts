import { sveltekit } from '@sveltejs/kit/vite';
import tailwindcss from '@tailwindcss/vite';
import basicSsl from '@vitejs/plugin-basic-ssl';
import sqlocalPlugin from 'sqlocal/vite';
import { defineConfig } from 'vite';

export default defineConfig({
	plugins: [tailwindcss(), sveltekit(), sqlocalPluginPatch(), basicSsl()],
	server: {
		https: true,
	},
});

function sqlocalPluginPatch(): ReturnType<typeof sqlocalPlugin> {
	const plugin = sqlocalPlugin();
	return {
		...plugin,
		configurePreviewServer: plugin.configureServer as any,
	};
}
