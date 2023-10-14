import "./Asset/CSS/Window.css";

export const Mode = (await import("solid-js")).createSignal(
	window.settings.mode
);

export const {
	default: {
		appWindow: { setIgnoreCursorEvents, label },
	},
}: {
	default: { appWindow: WebviewWindow };
} = await import("@tauri-apps/api/window");

await setIgnoreCursorEvents(true);

await (
	await import("@tauri-apps/api/event")
).listen("mode", async (event: { payload: { message: { Mode: string } } }) => {
	Mode[1](event.payload.message.Mode);
});

(await import("solid-js/web")).render(
	() => (
		<div class="Window" data-label={label} data-mode={Mode[0]()}>
			<For each={["BottomLeft", "BottomRight", "TopLeft", "TopRight"]}>
				{(corner: string) => <Corner id={corner} />}
			</For>
		</div>
	),
	document.getElementById("window") as HTMLElement
);

export const { default: Corner } = await import("./Element/Corner.jsx");

export const { For } = await import("solid-js");

import type { WebviewWindow } from "@tauri-apps/api/window";
