import { invoke } from "@tauri-apps/api/core";
import { getCurrentWebviewWindow } from "@tauri-apps/api/webviewWindow";
import { createEffect, createSignal, For, onMount, Show } from "solid-js";
import { render } from "solid-js/web";

import "./Asset/CSS/Controls.css";

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

const appWindow = getCurrentWebviewWindow();

onMount(() => {
	void call("state");
});

/** A range input that reports once, on release, so one drag is one write. */
function Slider(properties: { value: number; maximum: number; onCommit: (value: number) => void }) {
	const [value, setValue] = createSignal(properties.value);

	createEffect(() => setValue(properties.value));

	return (
		<div class="Slider">
			<input
				type="range"
				min="0"
				max={properties.maximum}
				step="1"
				value={value()}
				onInput={(event) => setValue(Number(event.currentTarget.value))}
				onChange={() => properties.onCommit(value())}
			/>
			<span class="Value">{value()}px</span>
		</div>
	);
}

render(
	() => (
		<main class="Controls">
			<header>
				<h1>Round</h1>
				<button class="Close" title="Close" onClick={() => appWindow.close()}>
					✕
				</button>
			</header>

			<Show when={State()} fallback={<p class="Empty">Reading settings...</p>}>
				{(state) => (
					<>
						<section>
							<h2>Appearance</h2>
							<div class="Segmented">
								<button
									classList={{ Active: state().mode === "dark" }}
									onClick={() => call("set_mode", { mode: "dark" })}
								>
									Dark
								</button>
								<button
									classList={{ Active: state().mode === "light" }}
									onClick={() => call("set_mode", { mode: "light" })}
								>
									Light
								</button>
							</div>
						</section>

						<section>
							<h2>Every monitor</h2>
							<Slider
								value={state().size}
								maximum={state().maximum}
								onCommit={(value) => call("set_size", { value })}
							/>
							<p class="Hint">The default. A monitor with its own value keeps it.</p>
						</section>

						<section>
							<h2>Per monitor</h2>
							<For each={state().monitors}>
								{(monitor) => (
									<div class="Monitor">
										<div class="Head">
											<span class="Name">{monitor.name}</span>
											<Show when={monitor.overridden}>
												<span class="Badge">own value</span>
											</Show>
											<button
												class="Link"
												onClick={() => call("reset", { label: monitor.label })}
											>
												Reset
											</button>
										</div>
										<Slider
											value={monitor.size}
											maximum={state().maximum}
											onCommit={(value) => call("set_size", { label: monitor.label, value })}
										/>
									</div>
								)}
							</For>
						</section>

						<section>
							<h2>Overlay</h2>
							<div class="Segmented">
								<button
									classList={{ Active: !state().hidden }}
									onClick={() => call("set_hidden", { hidden: false })}
								>
									Show
								</button>
								<button
									classList={{ Active: state().hidden }}
									onClick={() => call("set_hidden", { hidden: true })}
								>
									Hide
								</button>
							</div>
							<label class="Toggle">
								<input
									type="checkbox"
									checked={state().dock_hidden}
									onChange={(event) => call("set_dock", { hidden: event.currentTarget.checked })}
								/>
								<span>Hide the dock icon</span>
							</label>
						</section>
					</>
				)}
			</Show>
		</main>
	),
	document.getElementById("controls") as HTMLElement,
);
