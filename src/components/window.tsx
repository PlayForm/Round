import { appWindow } from "@tauri-apps/api/window";

import type { Component } from "solid-js";
import { createSignal, For, lazy } from "solid-js";
import { render } from "solid-js/web";
import "./../assets/css/window.css";

const Corner = lazy(() => import("./corner"));

const Window: Component = () => {
	const [corners] = createSignal([
		{ id: "top_left" },
		{ id: "top_right" },
		{ id: "bottom_right" },
		{ id: "bottom_left" },
	]);

	return (
		<div class="Window" data-label={appWindow.label}>
			<For each={corners()}>
				{(corner, i) => <Corner id={corner.id} />}
			</For>
		</div>
	);
};

render(() => <Window />, document.getElementById("box") as HTMLElement);
