// @ts-ignore
import { listen } from "@tauri-apps/api/event";
// @ts-ignore
import { createSignal, mergeProps } from "solid-js";

import "../assets/css/corner.css";

const [size, setSize] = createSignal(23);

await listen(
	"set-size",
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
