import { invoke } from "@tauri-apps/api/core";
import { getCurrentWebviewWindow } from "@tauri-apps/api/webviewWindow";
import { createEffect, createSignal, Index, onMount, Show } from "solid-js";

import Context from "../Context.js";
import Fit from "../Fn/Fit.js";
import Entry from "../Fn/Entry.js";
import Where from "../Fn/Where.js";
import type { State } from "../Type/Panel.js";

import Row from "./Row.jsx";
import Setting from "./Setting.jsx";
import Slider from "./Slider.jsx";

/** The panel: the scope selector, one row per setting, and the two controls that
 *  are not per monitor.
 *
 *  The state, the command that writes it and the selected scope are created here
 *  and handed down through a context, so they are owned by this component: the
 *  effects below are disposed with it, and no signal outlives the panel. */
export default () => {
	const Window = getCurrentWebviewWindow();

	/** The panel's copy of the Rust state; every command returns the new one. */
	const [State, setState] = createSignal<State>();

	/** Which monitor the settings below apply to, in the select's own terms: a
	 *  monitor's label, or the empty string for the "All monitors" entry.
	 *
	 *  The empty string is the value of the select's first entry, so the control
	 *  needs no translation of its own, and `Scope` is the single place it becomes
	 *  the absent label that every command already reads as "every monitor". */
	const [Selected, setSelected] = createSignal("");

	/** The selected monitor's label, or `undefined` for every monitor at once. */
	const Scope = () => Selected() || undefined;

	const call = async (command:string, args?:Record<string, unknown>) => {
		try {
			setState(await invoke<State>(command, args));
		} catch (Error) {
			console.error(`${command} failed`, Error);
		}
	};

	/** The panel itself: the window hugs it, the way a menu hugs its items. A
	 *  transparent window still swallows clicks, so empty space under the content
	 *  must not exist or the panel would eat clicks meant for the desktop. */
	let Element:HTMLDivElement | undefined;

	/** The scope selector, so the selection can be re-stated after a redraw. */
	let Picker:HTMLSelectElement | undefined;

	/** A scope whose monitor is gone - unplugged while the panel was open, or the
	 *  panel left open across a display change - falls back to every monitor: the
	 *  rows below would otherwise keep writing to a monitor that is no longer
	 *  there. */
	createEffect(() => {
		const label = Scope();

		if (label !== undefined && !(State()?.monitors ?? []).some((monitor) => monitor.label === label)) {
			setSelected("");
		}
	});

	/** Re-state the selection whenever the panel redraws.
	 *
	 *  The select is a native control holding a value of its own, and `<Index>`
	 *  reuses its option nodes: a select whose options were rebuilt under it can
	 *  fall back to its first entry while the rows below still show the old scope.
	 *  Writing the value the panel holds costs one property assignment and makes the
	 *  two agree whichever of them moved. */
	createEffect(() => {
		const value = Selected();

		State();

		if (Picker) Picker.value = value;
	});

	/** Size the window to the panel whenever what it draws changes. */
	createEffect(() => {
		State();
		Scope();

		const Measured = Element;

		if (Measured) requestAnimationFrame(() => void Fit(Measured, Window));
	});

	onMount(() => {
		void call("state");
	});

	return (
		<Context.Provider value={{ State, call, Selected, setSelected, Scope }}>
			<div class="Panel" ref={Element}>
				<Show when={State()} fallback={<p class="Empty">Reading settings...</p>}>
					{(state) => (
						<>
							<div class="Section">
								<h2 class="Title">Scope</h2>

								<select
									class="Picker"
									ref={Picker}
									title="Which monitors the settings below apply to"
									value={Selected()}
									onChange={(event) => setSelected(event.currentTarget.value)}>
									<option value="">All monitors</option>

									<Index each={state().monitors}>
										{(monitor, index) => (
											<option value={monitor().label}>{Entry(monitor(), index)}</option>
										)}
									</Index>
								</select>

								<p class="Where">{Where(state(), Scope())}</p>
							</div>

							<Setting
								title="Radius"
								word="radius"
								value={state().radius}
								monitors={state().monitors}
								scope={Scope()}
								pick={(monitor) => monitor.radius}
								set={(label, value) =>
									call("set_size", label === undefined ? { value } : { label, value })
								}
								control={(value, commit) => (
									<>
										<Slider value={value()} maximum={state().maximum} onCommit={commit} />
										<span class="Number">{value()}</span>
									</>
								)}
							/>

							<Setting
								title="Ink"
								word="ink"
								value={state().ink}
								monitors={state().monitors}
								scope={Scope()}
								pick={(monitor) => monitor.ink}
								set={(label, mode) =>
									call("set_mode", label === undefined ? { mode } : { label, mode })
								}
								control={(value, commit) => (
									<>
										<button classList={{ On:value() === "dark" }} onClick={() => commit("dark")}>
											Dark
										</button>
										<button
											classList={{ On:value() === "light" }}
											onClick={() => commit("light")}>
											Light
										</button>
									</>
								)}
							/>

							<Setting
								title="Visibility"
								word="hidden"
								value={state().hidden}
								monitors={state().monitors}
								scope={Scope()}
								pick={(monitor) => monitor.hidden}
								set={(label, hidden) =>
									call("set_hidden", label === undefined ? { hidden } : { label, hidden })
								}
								control={(value, commit) => (
									<>
										<button classList={{ On:!value() }} onClick={() => commit(false)}>
											Show
										</button>
										<button classList={{ On:value() }} onClick={() => commit(true)}>
											Hide
										</button>
									</>
								)}
							/>

							<hr class="Divider" />

							<div class="Section">
								<Row
									label="Dock icon"
									hint="The dock icon belongs to the application, not to a monitor">
									<input
										type="checkbox"
										checked={!state().dock_hidden}
										onChange={(event) =>
											call("set_dock", { hidden: !event.currentTarget.checked })
										}
									/>
								</Row>
								<p class="Note">
									One icon for the whole application - there is no per-monitor dock icon.
								</p>
							</div>

							<button class="Quit" onClick={() => invoke("quit")}>
								Quit Round
							</button>
						</>
					)}
				</Show>
			</div>
		</Context.Provider>
	);
};
