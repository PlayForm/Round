module.exports = {
	plugins: [
		require("postcss-import"),
		require("postcss-url"),
		require("tailwindcss/nesting"),
		require("tailwindcss")("./tailwind.config.js"),
		require("postcss-combine-media-query"),
		require("postcss-combine-duplicated-selectors")({
			removeDuplicatedProperties: true,
			removeDuplicatedValues: false,
		}),
		require("autoprefixer"),
		require("cssnano")({ preset: "advanced" }),
		require("postcss-reporter"),
	],
};
