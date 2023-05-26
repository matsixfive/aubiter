import { sveltekit } from '@sveltejs/kit/vite';
import { defineConfig } from 'vite';
import vitePluginWasmPack from 'vite-plugin-wasm-pack';

export default defineConfig({
	plugins: [sveltekit(), vitePluginWasmPack("./wasm/gravity")]
});
