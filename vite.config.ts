import { resolve } from "path";
import type { UserConfig } from "vite";
import solidPlugin from "vite-plugin-solid";

export default (): UserConfig => {
	return {
		plugins: [solidPlugin()],
		build: {
			target: "esnext",
			rollupOptions: {
				input: {
					window: resolve("src/windows/window.html"),
				},
			},
		},
	};
};
