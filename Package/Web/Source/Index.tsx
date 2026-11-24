import { listen } from "@tauri-apps/api/event";
import { getCurrentWebviewWindow } from "@tauri-apps/api/webviewWindow";
import { createSignal, For, onCleanup, onMount } from "solid-js";
import { render } from "solid-js/web";

import "./Asset/CSS/Window.css";

import Corner from "./Element/Corner.jsx";
import type { ModeMessage, SizeMessage } from "./Type/Settings.js";

const Corners = ["BottomLeft", "BottomRight", "TopLeft", "TopRight"];

/**
 * One overlay: four corners, drawn in this monitor's ink at this monitor's radius.
 *
 * Everything reactive lives inside the component, so the signals have an owner and
 * the listeners have a cleanup. Nothing runs at module scope: a signal created
 * there is a singleton with no owner, and a listener registered there can never be
 * taken down.
 */
function Overlay() {
	const Current = getCurrentWebviewWindow();

	/** The ink this monitor's corners are drawn in. */
	const [Mode, setMode] = createSignal(window.settings.mode);

	/** The corner radius this monitor is drawn with, in logical pixels. */
	const [Radius, setRadius] = createSignal(window.settings.size);

	onMount(() => {
		void Current.setIgnoreCursorEvents(true);

		/**
		 * `listen` without a target registers a GLOBAL listener - the API defaults
		 * to `{ kind: "Any" }` - and a targeted `emit_to(label, ...)` never matches
		 * that: the Rust filter compares labels and a candidate of `Any` falls
		 * through to `false`, so the event reaches nobody. Naming this window is
		 * what makes the pair match, and what keeps one monitor's ink off another
		 * monitor's screen.
		 */
		const Target = { target: Current.label };

		const Unlisten: Array<() => void> = [];

		void listen<ModeMessage>(
			"mode",
			({ payload }) => setMode(payload.message.Mode),
			Target,
		).then((Off) => Unlisten.push(Off));

		void listen<SizeMessage>(
			"size",
			({ payload }) => setRadius(payload.message.Size),
			Target,
		).then((Off) => Unlisten.push(Off));

		onCleanup(() => Unlisten.forEach((Off) => Off()));
	});

	return (
		<div
			class="Window"
			data-label={Current.label}
			data-mode={Mode()}
			style={`--Corner: ${Radius()}px`}>
			<For each={Corners}>{(corner) => <Corner id={corner} />}</For>
		</div>
	);
}

render(() => <Overlay />, document.getElementById("window") as HTMLElement);
