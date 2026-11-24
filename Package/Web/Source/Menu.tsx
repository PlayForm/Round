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

/** One monitor, with every setting already resolved for it. */
type Monitor = {
	label: string;
	name: string;
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

/** One row: a label, that row's controls, and the reset that undoes them.
 *
 *  `override` marks a monitor that carries a value of its own. The mark is
 *  drawn inside the label's own column, after the name, so it survives a name
 *  too long for the column and every row's controls still line up.
 */
function Row(properties: {
	label: string;
	hint: string;
	override?: boolean;
	reset: string;
	onReset: () => void;
	children: JSX.Element;
}) {
	return (
		<label class="Row">
			<span title={properties.hint}>
				<span class="Name">{properties.label}</span>
				<Show when={properties.override}>
					<span class="Mark" title="This monitor has a value of its own; the reset drops it">
						•
					</span>
				</Show>
			</span>
			{properties.children}
			<button class="Reset" title={properties.reset} onClick={properties.onReset}>
				↺
			</button>
		</label>
	);
}

/** One per-monitor setting: the "All monitors" row, then a row per monitor.
 *
 *  The three sections differ only in their controls and in the command behind
 *  them, so the rows are built once here: a setting cannot end up with a default
 *  row that resets differently from its monitor rows, or with a mark on one
 *  section and not another.
 */
function Setting<T>(properties: {
	title: string;
	/** The value a monitor without an override of its own is drawn with. */
	value: T;
	monitors: Monitor[];
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
	return (
		<div class="Section">
			<h2 class="Title">{properties.title}</h2>

			<Row
				label="All monitors"
				hint="Every monitor without a value of its own"
				reset="Back to the default and drop every monitor's override"
				onReset={() => call("reset", { setting: properties.word })}
			>
				{properties.control(
					() => properties.value,
					(next) => properties.set(undefined, next),
				)}
			</Row>

			<Index each={properties.monitors}>
				{(monitor) => (
					<Row
						label={monitor().name}
						hint={monitor().name}
						override={properties.pick(monitor()).origin === "override"}
						reset="Back to the default"
						onReset={() => call("reset", { setting: properties.word, label: monitor().label })}
					>
						{properties.control(
							() => properties.pick(monitor()).value,
							(next) => properties.set(monitor().label, next),
						)}
					</Row>
				)}
			</Index>
		</div>
	);
}

onMount(() => {
	void call("state");
});

render(
	() => (
		<div class="Panel" ref={Panel}>
			<Show when={State()} fallback={<p class="Empty">Reading settings...</p>}>
				{(state) => (
					<>
						<Setting
							title="Radius"
							word="radius"
							value={state().radius}
							monitors={state().monitors}
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

						<div class="Section">
							<label class="Row">
								<span title="The dock icon belongs to the application, not to a monitor">
									<span class="Name">Dock icon</span>
								</span>
								<input
									type="checkbox"
									checked={!state().dock_hidden}
									onChange={(event) => call("set_dock", { hidden: !event.currentTarget.checked })}
								/>
							</label>
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
