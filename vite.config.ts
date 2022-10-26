import { resolve } from "path";
import type { UserConfig } from "vite";
import solidPlugin from "vite-plugin-solid";

export default (): UserConfig => ({
	plugins: [solidPlugin()],
	server: {
		port: 3000,
	},
	build: {
		target: "esnext",
		rollupOptions: {
			input: {
				window: resolve("src/windows/window.html"),
			},
		},
	},
});
