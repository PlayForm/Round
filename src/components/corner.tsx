// @ts-ignore
import { listen } from "@tauri-apps/api/event";
// @ts-ignore
import { createSignal, mergeProps } from "solid-js";

import "../assets/css/corner.css";

const [size, setSize] = createSignal(23);

await listen(
	"boot",
	async (event: {
		payload: {
			message: string;
		};
	}) => {
		console.log(event.payload.message);
	}
);

await listen(
	"set-size",
	async (event: {
		payload: {
			message: number;
		};
	}) => {
		console.log(event.payload.message);
		setSize(event.payload.message);
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
