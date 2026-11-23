// PostCSS runs from a `type: module` package, and the current cssnano /
// postcss-combine-duplicated-selectors releases are ESM-only. `require()`
// therefore returns a module namespace object, not a callable plugin - which is
// exactly the `require(...) is not a function` failure this file replaces.
import autoprefixer from "autoprefixer";
import combineDuplicatedSelectors from "postcss-combine-duplicated-selectors";
import combineMediaQuery from "postcss-combine-media-query";
import cssnano from "cssnano";
import importPlugin from "postcss-import";
import nesting from "tailwindcss/nesting/index.js";
import reporter from "postcss-reporter";
import tailwind from "tailwindcss";
import url from "postcss-url";

export default {
	plugins: [
		importPlugin(),
		url(),
		nesting(),
		tailwind("./tailwind.config.js"),
		combineMediaQuery(),
		combineDuplicatedSelectors({
			removeDuplicatedProperties: true,
			removeDuplicatedValues: false,
		}),
		autoprefixer(),
		cssnano({ preset: "advanced" }),
		reporter(),
	],
};
