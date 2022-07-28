import { resolve } from "path";
import { defineConfig } from "vite";
import solidPlugin from "vite-plugin-solid";

export default defineConfig({
	plugins: [solidPlugin()],
	server: {
		port: 3000,
	},
	build: {
		target: "esnext",
		rollupOptions: {
			input: {
				window: resolve(__dirname, "src/windows/window.html"),
			},
		},
	},
});
