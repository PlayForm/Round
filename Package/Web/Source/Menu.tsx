import { invoke } from "@tauri-apps/api/core";
import { LogicalSize } from "@tauri-apps/api/dpi";
import { getCurrentWebviewWindow } from "@tauri-apps/api/webviewWindow";
import { createEffect, createSignal, For, onMount, Show } from "solid-js";
import { render } from "solid-js/web";

import "./Asset/CSS/Menu.css";

type Monitor = {
	label: string;
	name: string;
	size: number;
	overridden: boolean;
};

type State = {
	size: number;
	mode: "dark" | "light";
	hidden: boolean;
	dock_hidden: boolean;
	maximum: number;
	monitors: Monitor[];
};

/** The panel's copy of the Rust state; every command returns the new one. */
const [State, setState] = createSignal<State>();

async function call(command: string, args?: Record<string, unknown>) {
	try {
		setState(await invoke<State>(command, args));
	} catch (error) {
		console.error(`${command} failed`, error);
	}
}

onMount(() => {
	void call("state");
});

const appWindow = getCurrentWebviewWindow();

/** The window hugs the panel, the way a menu hugs its items: a transparent
 *  window still swallows clicks, so empty space under the content must not
 *  exist or the panel would eat clicks meant for the desktop. */
let Panel: HTMLDivElement | undefined;

createEffect(() => {
	State();

	requestAnimationFrame(() => {
		if (!Panel) return;

		const height = Math.ceil(Panel.getBoundingClientRect().height) + 12;

		void appWindow.setSize(new LogicalSize(320, height));
	});
});

/** A range input that reports once, on release, so one drag is one write. */
function Slider(properties: { value: number; maximum: number; onCommit: (value: number) => void }) {
	const [value, setValue] = createSignal(properties.value);

	createEffect(() => setValue(properties.value));

	return (
		<input
			type="range"
			min="0"
			max={properties.maximum}
			step="1"
			value={value()}
			onInput={(event) => setValue(Number(event.currentTarget.value))}
			onChange={() => properties.onCommit(value())}
		/>
	);
}

render(
	() => (
		<div class="Panel" ref={Panel}>
			<Show when={State()} fallback={<p class="Empty">Reading settings...</p>}>
				{(state) => (
					<>
						<label class="Row">
							<span>All monitors</span>
							<Slider
								value={state().size}
								maximum={state().maximum}
								onCommit={(value) => call("set_size", { value })}
							/>
							<span class="Number">{state().size}</span>
						</label>

						<For each={state().monitors}>
							{(monitor) => (
								<label class="Row">
									<span title={monitor.name}>{monitor.name}</span>
									<Slider
										value={monitor.size}
										maximum={state().maximum}
										onCommit={(value) => call("set_size", { label: monitor.label, value })}
									/>
									<span class="Number">{monitor.size}</span>
									<button
										class="Reset"
										title="Back to the default"
										onClick={() => call("reset", { label: monitor.label })}
									>
										↺
									</button>
								</label>
							)}
						</For>

						<div class="Row">
							<span>Appearance</span>
							<button
								classList={{ On: state().mode === "dark" }}
								onClick={() => call("set_mode", { mode: "dark" })}
							>
								Dark
							</button>
							<button
								classList={{ On: state().mode === "light" }}
								onClick={() => call("set_mode", { mode: "light" })}
							>
								Light
							</button>
						</div>

						<div class="Row">
							<span>Overlay</span>
							<button
								classList={{ On: !state().hidden }}
								onClick={() => call("set_hidden", { hidden: false })}
							>
								Show
							</button>
							<button
								classList={{ On: state().hidden }}
								onClick={() => call("set_hidden", { hidden: true })}
							>
								Hide
							</button>
						</div>

						<label class="Row">
							<span>Dock icon</span>
							<input
								type="checkbox"
								checked={!state().dock_hidden}
								onChange={(event) => call("set_dock", { hidden: !event.currentTarget.checked })}
							/>
						</label>

						<button class="Quit" onClick={() => invoke("quit")}>
							Quit Round
						</button>
					</>
				)}
			</Show>
		</div>
	),
	document.getElementById("menu") as HTMLElement,
);
