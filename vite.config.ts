import { defineConfig } from "vite";
import solidPlugin from "vite-plugin-solid";

export default defineConfig({
	plugins: [solidPlugin()],
	clearScreen: false,
	server: {
		port: 1420,
		strictPort: true,
		hmr: {
			overlay: false,
		},
	},
	envPrefix: ["VITE_", "TAURI_"],
	build: {
		target: "chrome105",
		minify: "esbuild",
		sourcemap: !!process.env.TAURI_DEBUG,
	},
});
