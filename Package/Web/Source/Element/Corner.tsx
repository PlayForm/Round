import { listen } from "@tauri-apps/api/event";
import { createSignal } from "solid-js";

import "../Asset/CSS/Corner.css";

import type { SizeMessage } from "../Type/Settings.js";

type CornerProperty = {
	id?: string;
};

/** Corner radius, kept in sync with the tray's Increase/Decrease/Reset. */
const Size = createSignal(window.settings.size);

await listen<SizeMessage>("size", ({ payload }) => {
	Size[1](payload.message.Size);
});

export default (Property: CornerProperty) => (
	<div
		class="Corner"
		data-corner={Property.id ?? "Default"}
		style={{ "--Corner": `${Size[0]()}px` }}
	/>
);
