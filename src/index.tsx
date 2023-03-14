import { listen } from "@tauri-apps/api/event";
import { appWindow } from "@tauri-apps/api/window";
import type { Component } from "solid-js";
import { createSignal, For } from "solid-js";
import { render } from "solid-js/web";

import "./assets/css/window.css";

import Corner from "./components/corner.jsx";

// @ts-expect-error
const [mode, setMode] = createSignal(window.settings.mode);

await appWindow.setIgnoreCursorEvents(true);

await listen(
	"mode",
	async (event: { payload: { message: { Mode: string } } }) => {
		setMode(event.payload.message.Mode);
	}
);

const Window: Component = () => {
	return (
		<div class="Window" data-label={appWindow.label} data-mode={mode()}>
			<For
				each={["top_left", "top_right", "bottom_right", "bottom_left"]}>
				{(corner: string) => <Corner id={corner} />}
			</For>
		</div>
	);
};

render(() => <Window />, document.getElementById("window") as HTMLElement);
