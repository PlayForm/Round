import { invoke } from "@tauri-apps/api/core";
import { LogicalPosition, LogicalSize } from "@tauri-apps/api/dpi";
import { currentMonitor } from "@tauri-apps/api/window";
import { getCurrentWebviewWindow } from "@tauri-apps/api/webviewWindow";
import { createEffect, createSignal, Index, onMount, Show, type JSX } from "solid-js";
import { render } from "solid-js/web";

import "./Asset/CSS/Menu.css";

/** One setting as one monitor sees it: the value it is drawn with, and whether
 *  that value is its own. A value alone cannot say the second half - a monitor
 *  tuned to exactly the default is still an override - so Rust sends both. */
type Value<T> = { value: T; origin: "inherit" | "override" };

/** One monitor, with every setting already resolved for it.
 *
 *  `width`/`height` are the display's own pixels, and they are the only facts
 *  here that no setting reads: a name does not identify a display - two of them
 *  routinely arrive with the same one - so the panel needs a second fact to tell
 *  the entries of the scope selector apart. */
type Monitor = {
	label: string;
	name: string;
	width: number;
	height: number;
	radius: Value<number>;
	ink: Value<"dark" | "light">;
	hidden: Value<boolean>;
};

/** Everything the panel draws: each per-monitor setting's default, and every
 *  monitor's own answer for the three of them. The dock icon is the one control
 *  that is not per monitor, and travels once. */
type State = {
	radius: number;
	maximum: number;
	ink: "dark" | "light";
	hidden: boolean;
	dock_hidden: boolean;
	monitors: Monitor[];
};

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

async function call(command: string, args?: Record<string, unknown>) {
	try {
		setState(await invoke<State>(command, args));
	} catch (error) {
		console.error(`${command} failed`, error);
	}
}

const appWindow = getCurrentWebviewWindow();

/** The panel itself: the window hugs it, the way a menu hugs its items. A
 *  transparent window still swallows clicks, so empty space under the content
 *  must not exist or the panel would eat clicks meant for the desktop. */
let Panel: HTMLDivElement | undefined;

/** The scope selector, so the selection can be re-stated after a redraw. */
let Picker: HTMLSelectElement | undefined;

/** The width the panel is drawn at, in logical pixels - `Menu::Width`. */
const Width = 320;

/** The transparent margin the window keeps around the panel, on each side. */
const Margin = 6;

/** Logical pixels of the work area kept clear of the panel: the gap it hangs
 *  below the tray icon by, plus the margin above and below it. A panel at the
 *  cap therefore ends 6px short of the work area's edge rather than on it. */
const Reserve = 6 + 2 * Margin;

/** A work area, in logical pixels. */
type Work = { Top: number; Height: number; Scale: number };

/** The work area of the monitor the panel is on.
 *
 *  `screen.availHeight` is the same fact without an IPC round trip - the webview
 *  reports the work area of the display the window is on - so a panel whose
 *  monitor cannot be read is still capped at the screen rather than left to grow
 *  past it. `undefined` is a platform that answers neither: the panel then keeps
 *  the height its content asks for, which is what it did before this cap.
 */
async function Screen(): Promise<Work | undefined> {
	const monitor = await currentMonitor().catch(() => null);

	if (!monitor) {
		const Height = window.screen.availHeight;

		return Height > 0
			? { Top: 0, Height: Math.floor(Height), Scale: window.devicePixelRatio || 1 }
			: undefined;
	}

	const Scale = monitor.scaleFactor;
	const Height = monitor.workArea.size.height / Scale;

	return Height > 0 ? { Top: monitor.workArea.position.y / Scale, Height: Math.floor(Height), Scale } : undefined;
}

/** Size the window to the panel, and keep the panel inside the work area.
 *
 *  Rust places the panel from the size it last had, which is not the size it is
 *  about to be drawn at: the first open after an update, and any open after a
 *  monitor was added, grow the panel after it has been placed. So the panel also
 *  pulls itself back inside the work area - a panel that grew under a tray icon
 *  with room only for the old height would otherwise hang past the bottom edge
 *  with its Quit button off screen.
 *
 *  Every step here is a nicety: a panel that could not be measured or moved is
 *  still a panel, so no failure aborts the render.
 */
async function Fit() {
	if (!Panel) return;

	const Area = await Screen();

	// The cap goes on before the measurement, not after it: the panel scrolls
	// inside the window, so the height it measures is the height it is drawn at.
	Panel.style.maxHeight = Area ? `${Math.max(Area.Height - Reserve, 2 * Margin)}px` : "";

	const Height = Math.ceil(Panel.getBoundingClientRect().height) + 2 * Margin;

	await appWindow
		.setSize(new LogicalSize(Width, Height))
		.catch((error) => console.warn("round: the panel could not be resized", error));

	if (!Area) return;

	const Position = await appWindow.outerPosition().catch(() => null);

	if (!Position) return;

	const Y = Position.y / Area.Scale;
	const Top = Math.min(Math.max(Y, Area.Top), Math.max(Area.Top, Area.Top + Area.Height - Height));

	if (Math.abs(Top - Y) < 1) return;

	await appWindow
		.setPosition(new LogicalPosition(Position.x / Area.Scale, Top))
		.catch((error) => console.warn("round: the panel could not be moved back on screen", error));
}

createEffect(() => {
	State();
	Scope();

	requestAnimationFrame(() => void Fit());
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

/** A monitor's resolution, as the panel prints it. */
const Size = (monitor: Monitor) => `${monitor.width} × ${monitor.height}`;

/** How one monitor is named in the scope selector: its own name, then the index
 *  the system gave it and the pixels it draws at, because a name alone does not
 *  identify a display - two of them routinely arrive with the same one, and the
 *  select truncates whatever does not fit. */
const Entry = (monitor: Monitor, index: number) => `${index + 1}. ${monitor.name} · ${Size(monitor)}`;

/** The scope in full, spelled out under the selector.
 *
 *  The closed select shows one truncated line, and the part it truncates first
 *  is the name: two displays both reading "Monitor #26..." is exactly the state
 *  the selector has to be usable in. So the panel repeats the selection here,
 *  where it can wrap, with the facts that tell the entries apart. */
const Where = (state: State, label: string | undefined) => {
	const index = label === undefined ? -1 : state.monitors.findIndex((monitor) => monitor.label === label);

	const monitor = index < 0 ? undefined : state.monitors[index];

	return monitor === undefined
		? `All monitors · ${state.monitors.length} connected`
		: `${index + 1} of ${state.monitors.length} · ${monitor.name} · ${Size(monitor)}`;
};

/** One row: a label, that row's controls, and the reset that undoes them.
 *
 *  `mark` is the row's "own value" bullet, drawn inside the label's own column
 *  after the name so it survives a name too long for the column and every row's
 *  controls still line up. `reset` is absent on a row with nothing to reset.
 */
function Row(properties: {
	label: string;
	hint: string;
	mark?: string | undefined;
	reset?: string | undefined;
	onReset?: (() => void) | undefined;
	children: JSX.Element;
}) {
	return (
		<label class="Row">
			<span title={properties.hint}>
				<span class="Name">{properties.label}</span>
				<Show when={properties.mark}>
					{(mark) => (
						<span class="Mark" title={mark()}>
							•
						</span>
					)}
				</Show>
			</span>
			{properties.children}
			<Show when={properties.reset}>
				{(reset) => (
					<button class="Reset" title={reset()} onClick={() => properties.onReset?.()}>
						↺
					</button>
				)}
			</Show>
		</label>
	);
}

/** One per-monitor setting, drawn once for the scope the panel is showing.
 *
 *  The monitor is the panel's navigation axis, not a row: a row per monitor
 *  would be three settings times one default plus every monitor, which is nine
 *  rows with two displays and three thousand with a thousand. So the selector at
 *  the top picks the scope and this draws one row for it - the default when the
 *  scope is "All monitors", that monitor's own value when it is one display.
 *
 *  All three settings come through here, so a setting cannot end up resetting
 *  differently from its neighbours, or marking an override on one row and not
 *  another.
 */
function Setting<T>(properties: {
	title: string;
	/** The value a monitor without an override of its own is drawn with. */
	value: T;
	monitors: Monitor[];
	/** The scope the panel is showing: a monitor's label, or `undefined` for
	 *  every monitor. */
	scope: string | undefined;
	/** One monitor's own copy of this setting. */
	pick: (monitor: Monitor) => Value<T>;
	/** Draws one row's controls for the value it is drawn with. */
	control: (value: () => T, commit: (next: T) => void) => JSX.Element;
	/** Writes the setting for one monitor, or for every monitor when no label
	 *  is given. */
	set: (label: string | undefined, next: T) => void;
	/** The word `reset` knows this setting by. */
	word: string;
}) {
	/** The monitor the panel is showing, when it is showing one. */
	const Shown = () => {
		const label = properties.scope;

		return label === undefined ? undefined : properties.monitors.find((monitor) => monitor.label === label);
	};

	/** The value this row is drawn with. */
	const Drawn = () => {
		const monitor = Shown();

		return monitor === undefined ? properties.value : properties.pick(monitor).value;
	};

	/** The mark this row carries, if any.
	 *
	 *  One monitor in scope marks its own value, which is the fact a value alone
	 *  cannot state. Every monitor in scope marks how many monitors a write here
	 *  would drop: writing the default clears every override, so the count is the
	 *  warning that the control itself does not give. */
	const Mark = () => {
		const monitor = Shown();

		if (monitor !== undefined) {
			return properties.pick(monitor).origin === "override"
				? "This monitor has a value of its own; the reset drops it"
				: undefined;
		}

		const count = properties.monitors.filter((each) => properties.pick(each).origin === "override").length;

		if (count === 0) return undefined;

		const monitors = count === 1 ? "1 monitor has a value" : `${count} monitors have values`;
		const dropped = count === 1 ? "it" : "them";

		return `${monitors} of their own; setting or resetting here drops ${dropped}`;
	};

	return (
		<Row
			label={properties.title}
			hint={
				Shown() === undefined
					? "The value every monitor without one of its own is drawn with"
					: "This monitor's own value, or the default it follows"
			}
			mark={Mark()}
			reset={
				Shown() === undefined
					? "Back to the default and drop every monitor's override"
					: "Back to the default, dropping only this monitor's override"
			}
			onReset={() =>
				call(
					"reset",
					Shown() === undefined
						? { setting: properties.word }
						: { setting: properties.word, label: properties.scope },
				)
			}
		>
			{properties.control(Drawn, (next) => properties.set(properties.scope, next))}
		</Row>
	);
}

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

onMount(() => {
	void call("state");
});

render(
	() => (
		<div class="Panel" ref={Panel}>
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
								onChange={(event) => setSelected(event.currentTarget.value)}
							>
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
							set={(label, value) => call("set_size", label === undefined ? { value } : { label, value })}
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
							set={(label, mode) => call("set_mode", label === undefined ? { mode } : { label, mode })}
							control={(value, commit) => (
								<>
									<button classList={{ On: value() === "dark" }} onClick={() => commit("dark")}>
										Dark
									</button>
									<button classList={{ On: value() === "light" }} onClick={() => commit("light")}>
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
							set={(label, hidden) => call("set_hidden", label === undefined ? { hidden } : { label, hidden })}
							control={(value, commit) => (
								<>
									<button classList={{ On: !value() }} onClick={() => commit(false)}>
										Show
									</button>
									<button classList={{ On: value() }} onClick={() => commit(true)}>
										Hide
									</button>
								</>
							)}
						/>

						<hr class="Divider" />

						<div class="Section">
							<Row label="Dock icon" hint="The dock icon belongs to the application, not to a monitor">
								<input
									type="checkbox"
									checked={!state().dock_hidden}
									onChange={(event) => call("set_dock", { hidden: !event.currentTarget.checked })}
								/>
							</Row>
							<p class="Note">One icon for the whole application - there is no per-monitor dock icon.</p>
						</div>

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
