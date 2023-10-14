import type { Settings } from "../Option/Index.js";

import "../Asset/CSS/Corner.css";

import { listen } from "@tauri-apps/api/event";
import { createSignal, mergeProps } from "solid-js";

declare global {
	interface Window {
		settings: Settings;
	}
}

const [size, setSize] = createSignal(window.settings.size);

await listen(
	"size",
	async (event: { payload: { message: { Size: number } } }) => {
		setSize(event.payload.message.Size);
	}
);

// biome-ignore lint/suspicious/noExplicitAny:
export default (props: any) => (
	<div
		class="Corner"
		data-corner={mergeProps({ id: "Default" }, props).id}
		style={{ "--corner-size": `${size()}px` }}
	/>
);
