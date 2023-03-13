import { listen } from "@tauri-apps/api/event";
import { appWindow } from "@tauri-apps/api/window";
import type { Component } from "solid-js";
import { createSignal, For } from "solid-js";
import { render } from "solid-js/web";
import Corner from "./components/corner";

import "./assets/css/window.css";

// // @ts-ignore
// import { Store } from "tauri-plugin-store-api";

// const store = new Store(".settings.dat");
// const storeMode: {
// 	value: string;
// } | null = await store.get("mode");

const [mode, setMode] = createSignal("dark");

await listen(
	"switch-mode",
	async (event: {
		payload: {
			message: string;
		};
	}) => {
		setMode(event.payload.message);
		// await store.set("mode", { value: mode() });
		// await store.save();
	}
);

const Window: Component = () => {
	const [corners] = createSignal([
		{ id: "top_left" },
		{ id: "top_right" },
		{ id: "bottom_right" },
		{ id: "bottom_left" },
	]);

	return (
		<div class="Window" data-label={appWindow.label} data-mode={mode()}>
			<For each={corners()}>{(corner) => <Corner id={corner.id} />}</For>
		</div>
	);
};

render(() => <Window />, document.getElementById("window") as HTMLElement);
