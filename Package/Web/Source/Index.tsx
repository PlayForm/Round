import { listen } from "@tauri-apps/api/event";
import { getCurrentWebviewWindow } from "@tauri-apps/api/webviewWindow";
import { createSignal, For } from "solid-js";
import { render } from "solid-js/web";

import "./Asset/CSS/Window.css";

import Corner from "./Element/Corner.jsx";
import type { ModeMessage } from "./Type/Settings.js";

/** The ink every corner is drawn in, kept in sync with the tray's Dark/Light. */
const Mode = createSignal(window.settings.mode);

const appWindow = getCurrentWebviewWindow();

await appWindow.setIgnoreCursorEvents(true);

await listen<ModeMessage>("mode", ({ payload }) => {
	Mode[1](payload.message.Mode);
});

const Corners = ["BottomLeft", "BottomRight", "TopLeft", "TopRight"];

render(
	() => (
		<div class="Window" data-label={appWindow.label} data-mode={Mode[0]()}>
			<For each={Corners}>{(corner) => <Corner id={corner} />}</For>
		</div>
	),
	document.getElementById("window") as HTMLElement,
);
