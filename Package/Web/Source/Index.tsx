import { listen } from "@tauri-apps/api/event";
import { invoke } from "@tauri-apps/api/core";
import { getCurrentWebviewWindow } from "@tauri-apps/api/webviewWindow";
import { createSignal, For } from "solid-js";
import { render } from "solid-js/web";

import "./Asset/CSS/Window.css";

import Corner from "./Element/Corner.jsx";
import type { ModeMessage, SizeMessage } from "./Type/Settings.js";

/** The ink this monitor's corners are drawn in. */
const Mode = createSignal(window.settings.mode);

/** The corner radius this monitor is drawn with, in logical pixels. */
const Radius = createSignal(window.settings.size);

const appWindow = getCurrentWebviewWindow();

await appWindow.setIgnoreCursorEvents(true);

/**
 * `listen` without a target registers a GLOBAL listener - the API defaults to
 * `{ kind: "Any" }` - and a targeted `emit_to(label, ...)` never matches that:
 * the Rust filter compares labels and a candidate of `Any` falls through to
 * `false`, so the event reaches nobody and the setting appears to do nothing.
 * Naming this window here is what makes the pair match, and it is what keeps one
 * monitor's ink off the other monitor's screen.
 */
const Target = { target: appWindow.label };

/** Debug: report what this overlay is actually rendering. The Rust side drops
 *  the line unless `ROUND_TRACE` is set, so a release build stays silent. */
const Report = () => {
	const Element = document.querySelector(".Window");

	void invoke("Report", {
		label: appWindow.label,
		mode: String(Element?.getAttribute("data-mode")),
		radius: Element === null ? "?" : getComputedStyle(Element).getPropertyValue("--Corner").trim(),
	});
};

await listen<ModeMessage>(
	"mode",
	({ payload }) => {
		Mode[1](payload.message.Mode);

		Report();
	},
	Target,
);

await listen<SizeMessage>(
	"size",
	({ payload }) => {
		Radius[1](payload.message.Size);

		Report();
	},
	Target,
);

const Corners = ["BottomLeft", "BottomRight", "TopLeft", "TopRight"];

render(
	() => (
		<div
			class="Window"
			data-label={appWindow.label}
			data-mode={Mode[0]()}
			style={`--Corner: ${Radius[0]()}px`}
		>
			<For each={Corners}>{(corner) => <Corner id={corner} />}</For>
		</div>
	),
	document.getElementById("window") as HTMLElement,
);

// Debug: the state after the first paint, before any event.
requestAnimationFrame(Report);
