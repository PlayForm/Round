import { fileURLToPath } from "node:url";

export default (await import("vite")).defineConfig({
	root: fileURLToPath(new URL(".", import.meta.url)),
	publicDir: "./Public",
	plugins: [(await import("vite-plugin-solid")).default()],
	clearScreen: false,
	server: {
		// Bind the IPv4 loopback explicitly: `localhost` resolves to `::1`
		// first on macOS, so a listener on the default host is unreachable from
		// anything that dials 127.0.0.1 (including Tauri's own health checks).
		host: "127.0.0.1",
		port: 1420,
		strictPort: true,
		hmr: {
			overlay: false,
		},
	},
	envPrefix: ["VITE_", "TAURI_"],
	build: {
		outDir: "Target",
		emptyOutDir: true,
		target: "chrome105",
		minify: process.env["TAURI_DEBUG"] ? false : "esbuild",
		sourcemap: !!process.env["TAURI_DEBUG"],
		rollupOptions: {
			// Two documents: `index.html` is the overlay every monitor renders,
			// `controls.html` is the panel the tray opens.
			input: {
				index: fileURLToPath(new URL("index.html", import.meta.url)),
				controls: fileURLToPath(new URL("controls.html", import.meta.url)),
			},
		},
	},
});
