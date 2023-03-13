// @ts-ignore
import { listen } from "@tauri-apps/api/event";
// @ts-ignore
import { createSignal, mergeProps } from "solid-js";

import "../assets/css/corner.css";

// @ts-ignore
// import { Store } from "tauri-plugin-store-api";
// const store = new Store(".settings.dat");

// const storeSize: {
// 	value: number;
// } | null = await store.get("size");

const [size, setSize] = createSignal(23);

await listen(
	"switch-size",
	async (event: {
		payload: {
			message: string;
		};
	}) => {
		switch (event.payload.message) {
			case "increase": {
				// rome-ignore lint/nursery/noPrecisionLoss:
				setSize((s: number) => s + 3);
				break;
			}

			case "decrease": {
				// rome-ignore lint/nursery/noPrecisionLoss:
				setSize((s: number) => (s - 3 === 0 ? s : s - 3));
				break;
			}

			case "reset": {
				setSize(23);
				break;
			}

			default:
				break;
		}

		// await store.set("size", { value: size() });
		// await store.save();
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
