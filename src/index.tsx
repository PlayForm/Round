import { listen } from "@tauri-apps/api/event";
import { appWindow } from "@tauri-apps/api/window";
import type { Component } from "solid-js";
import { createSignal, For } from "solid-js";
import { render } from "solid-js/web";
import Corner from "./components/corner";

import "./assets/css/window.css";

const [mode, setMode] = createSignal("dark");

await appWindow.setIgnoreCursorEvents(true);

await listen(
	"boot",
	async (event: {
		payload: {
			message: string;
		};
	}) => {
		console.log(event.payload.message);
	}
);

await listen(
	"set-mode",
	async (event: {
		payload: {
			message: string;
		};
	}) => {
		console.log(event.payload.message);
		setMode(event.payload.message);
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
			<For each={corners()}>
				{(corner: { id: string }) => <Corner id={corner.id} />}
			</For>
		</div>
	);
};

render(() => <Window />, document.getElementById("window") as HTMLElement);
