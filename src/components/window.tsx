import type { Component } from "solid-js";
import { createSignal, For } from "solid-js";
import { render } from "solid-js/web";

import { appWindow } from "@tauri-apps/api/window";

import "./../assets/css/window.css";
import Corner from "./corner";

const Window: Component = () => {
	const [corners] = createSignal([
		{ id: "top_left" },
		{ id: "top_right" },
		{ id: "bottom_right" },
		{ id: "bottom_left" },
	]);

	return (
		<div class="Window" data-label={appWindow.label}>
			<For each={corners()}>{(corner) => <Corner id={corner.id} />}</For>
		</div>
	);
};

render(() => <Window />, document.getElementById("box") as HTMLElement);
