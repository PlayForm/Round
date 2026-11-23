import { listen } from "@tauri-apps/api/event";
import { createSignal, mergeProps } from "solid-js";

import "../Asset/CSS/Corner.css";

import type { Settings } from "../Type/Settings.js";

declare global {
	interface Window {
		settings: Settings;
	}
}

type SizePayload = {
	message: { Size: number };
};

export const Size = createSignal(window.settings.size);

await listen<SizePayload>("size", ({ payload }) => {
	Size[1](payload.message.Size);
});

type CornerProperty = {
	id?: string;
};

export default (Property: CornerProperty) => (
	<div
		class="Corner"
		data-corner={mergeProps({ id: "Default" }, Property).id}
		style={{ "--Corner": `${Size[0]()}px` }}
	/>
);
