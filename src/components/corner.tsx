import { listen } from "@tauri-apps/api/event";
import { createSignal, mergeProps } from "solid-js";

import "../assets/css/corner.css";

const [size, setSize] = createSignal((window as Settings).settings.size);

await listen(
	"size",
	async (event: { payload: { message: { Size: number } } }) => {
		setSize(event.payload.message.Size);
	}
);

const Corner = (props: unknown) => (
	<div
		class="Corner"
		data-id={mergeProps({ id: "default" }, props).id}
		style={{ "--corner-size": `${size()}px` }}
	/>
);

export default Corner;
