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
	"set-mode",
	async (event: { payload: { message: { Mode: string } } }) => {
		setMode(event.payload.message.Mode);
	}
);

const Window: Component = () => {
	return (
		<div class="Window" data-label={appWindow.label} data-mode={mode()}>
			<For
				each={["top_left", "top_right", "bottom_right", "bottom_left"]}>
				{(corner) => <Corner id={corner} />}
			</For>
		</div>
	);
};

render(() => <Window />, document.getElementById("window") as HTMLElement);
