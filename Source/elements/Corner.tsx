import { listen } from "@tauri-apps/api/event";
import { createSignal, mergeProps } from "solid-js";
import type { Settings } from "../Option/Index.js";
import "../assets/css/Corner.css";

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

// rome-ignore lint/suspicious/noExplicitAny:
export default (props: any) => (
	<div
		class="Corner"
		data-corner={mergeProps({ id: "Default" }, props).id}
		style={{ "--corner-size": `${size()}px` }}
	/>
);
