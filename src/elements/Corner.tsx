import { listen } from "@tauri-apps/api/event";
import { createSignal } from "solid-js";

import "../assets/css/corner.css";

import type { Settings } from "../options/index.js";

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

export default () => (
	<div class="Corner" style={{ "--corner-size": `${size()}px` }} />
);
