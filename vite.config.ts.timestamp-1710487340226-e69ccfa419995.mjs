// vite.config.ts
var vite_config_default = (
	await import(
		"file:///D:/Developer/node_modules/.pnpm/vite@5.1.6_@types+node@20.11.27/node_modules/vite/dist/node/index.js"
	)
).defineConfig({
	publicDir: "./Public",
	plugins: [
		(
			await import(
				"file:///D:/Developer/node_modules/.pnpm/vite-plugin-solid@2.10.1_solid-js@1.8.15_vite@5.1.6/node_modules/vite-plugin-solid/dist/esm/index.mjs"
			)
		).default(),
	],
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
		outDir: "Target",
		target: "chrome105",
		minify: process.env["TAURI_DEBUG"] ? false : "esbuild",
		sourcemap: !!process.env["TAURI_DEBUG"],
	},
});
export { vite_config_default as default };
//# sourceMappingURL=data:application/json;base64,ewogICJ2ZXJzaW9uIjogMywKICAic291cmNlcyI6IFsidml0ZS5jb25maWcudHMiXSwKICAic291cmNlc0NvbnRlbnQiOiBbImNvbnN0IF9fdml0ZV9pbmplY3RlZF9vcmlnaW5hbF9kaXJuYW1lID0gXCJEOlxcXFxEZXZlbG9wZXJcXFxcQXBwbGljYXRpb25cXFxcUm91bmRlZENvcm5lcnNcXFxcQXBwbGljYXRpb25cIjtjb25zdCBfX3ZpdGVfaW5qZWN0ZWRfb3JpZ2luYWxfZmlsZW5hbWUgPSBcIkQ6XFxcXERldmVsb3BlclxcXFxBcHBsaWNhdGlvblxcXFxSb3VuZGVkQ29ybmVyc1xcXFxBcHBsaWNhdGlvblxcXFx2aXRlLmNvbmZpZy50c1wiO2NvbnN0IF9fdml0ZV9pbmplY3RlZF9vcmlnaW5hbF9pbXBvcnRfbWV0YV91cmwgPSBcImZpbGU6Ly8vRDovRGV2ZWxvcGVyL0FwcGxpY2F0aW9uL1JvdW5kZWRDb3JuZXJzL0FwcGxpY2F0aW9uL3ZpdGUuY29uZmlnLnRzXCI7ZXhwb3J0IGRlZmF1bHQgKGF3YWl0IGltcG9ydChcInZpdGVcIikpLmRlZmluZUNvbmZpZyh7XG5cdHB1YmxpY0RpcjogXCIuL1B1YmxpY1wiLFxuXHRwbHVnaW5zOiBbKGF3YWl0IGltcG9ydChcInZpdGUtcGx1Z2luLXNvbGlkXCIpKS5kZWZhdWx0KCldLFxuXHRjbGVhclNjcmVlbjogZmFsc2UsXG5cdHNlcnZlcjoge1xuXHRcdHBvcnQ6IDE0MjAsXG5cdFx0c3RyaWN0UG9ydDogdHJ1ZSxcblx0XHRobXI6IHtcblx0XHRcdG92ZXJsYXk6IGZhbHNlLFxuXHRcdH0sXG5cdH0sXG5cdGVudlByZWZpeDogW1wiVklURV9cIiwgXCJUQVVSSV9cIl0sXG5cdGJ1aWxkOiB7XG5cdFx0b3V0RGlyOiBcIlRhcmdldFwiLFxuXHRcdHRhcmdldDogXCJjaHJvbWUxMDVcIixcblx0XHRtaW5pZnk6IHByb2Nlc3MuZW52W1wiVEFVUklfREVCVUdcIl0gPyBmYWxzZSA6IFwiZXNidWlsZFwiLFxuXHRcdHNvdXJjZW1hcDogISFwcm9jZXNzLmVudltcIlRBVVJJX0RFQlVHXCJdLFxuXHR9LFxufSk7XG4iXSwKICAibWFwcGluZ3MiOiAiO0FBQXFWLElBQU8sdUJBQVMsTUFBTSxPQUFPLDhHQUFNLEdBQUcsYUFBYTtBQUFBLEVBQ3ZZLFdBQVc7QUFBQSxFQUNYLFNBQVMsRUFBRSxNQUFNLE9BQU8sK0lBQW1CLEdBQUcsUUFBUSxDQUFDO0FBQUEsRUFDdkQsYUFBYTtBQUFBLEVBQ2IsUUFBUTtBQUFBLElBQ1AsTUFBTTtBQUFBLElBQ04sWUFBWTtBQUFBLElBQ1osS0FBSztBQUFBLE1BQ0osU0FBUztBQUFBLElBQ1Y7QUFBQSxFQUNEO0FBQUEsRUFDQSxXQUFXLENBQUMsU0FBUyxRQUFRO0FBQUEsRUFDN0IsT0FBTztBQUFBLElBQ04sUUFBUTtBQUFBLElBQ1IsUUFBUTtBQUFBLElBQ1IsUUFBUSxRQUFRLElBQUksYUFBYSxJQUFJLFFBQVE7QUFBQSxJQUM3QyxXQUFXLENBQUMsQ0FBQyxRQUFRLElBQUksYUFBYTtBQUFBLEVBQ3ZDO0FBQ0QsQ0FBQzsiLAogICJuYW1lcyI6IFtdCn0K
