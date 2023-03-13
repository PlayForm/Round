import { defineConfig } from "vite";
import solidPlugin from "vite-plugin-solid";

export default defineConfig(async () => ({
	plugins: [solidPlugin()],
	clearScreen: false,
	server: {
		port: 1420,
		strictPort: true,
	},
	envPrefix: ["VITE_", "TAURI_"],
	build: {
		target:
			process.env.TAURI_PLATFORM == "windows" ? "chrome105" : "safari13",
		minify: "esbuild",
		sourcemap: !!process.env.TAURI_DEBUG,
	},
}));
