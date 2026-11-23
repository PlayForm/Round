import { listen } from "@tauri-apps/api/event";
import { getCurrentWebviewWindow } from "@tauri-apps/api/webviewWindow";
import { createSignal, For } from "solid-js";
import { render } from "solid-js/web";

import "./Asset/CSS/Window.css";

import Corner from "./Element/Corner.jsx";
import type { Settings } from "./Type/Settings.js";

declare global {
	interface Window {
		settings: Settings;
	}
}

type ModePayload = {
	message: { Mode: Settings["mode"] };
};

export const Mode = createSignal<Settings["mode"]>(window.settings.mode);

const appWindow = getCurrentWebviewWindow();

await appWindow.setIgnoreCursorEvents(true);

await listen<ModePayload>("mode", ({ payload }) => {
	Mode[1](payload.message.Mode);
});

render(
	() => (
		<div class="Window" data-label={appWindow.label} data-mode={Mode[0]()}>
			<For each={["BottomLeft", "BottomRight", "TopLeft", "TopRight"]}>
				{(corner) => <Corner id={corner} />}
			</For>
		</div>
	),
	document.getElementById("window") as HTMLElement,
);
