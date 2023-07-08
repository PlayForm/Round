import solidPlugin from "vite-plugin-solid";
import { defineConfig } from "vite";

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
