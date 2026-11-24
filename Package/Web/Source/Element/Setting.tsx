import { useContext, type JSX } from "solid-js";

import Context from "../Context.js";
import type { Monitor, Value } from "../Type/Panel.js";

import Row from "./Row.jsx";

/** One per-monitor setting, drawn once for the scope the panel is showing.
 *
 *  The monitor is the panel's navigation axis, not a row: a row per monitor would
 *  be three settings times one default plus every monitor, which is nine rows with
 *  two displays and three thousand with a thousand. So the selector at the top
 *  picks the scope and this draws one row for it - the default when the scope is
 *  "All monitors", that monitor's own value when it is one display.
 *
 *  All three settings come through here, so a setting cannot end up resetting
 *  differently from its neighbours, or marking an override on one row and not
 *  another.
 */
export default <T,>(properties:{
	title:string;
	/** The value a monitor without an override of its own is drawn with. */
	value:T;
	monitors:Monitor[];
	/** The scope the panel is showing: a monitor's label, or `undefined` for every
	 *  monitor. */
	scope:string | undefined;
	/** One monitor's own copy of this setting. */
	pick:(monitor:Monitor) => Value<T>;
	/** Draws one row's controls for the value it is drawn with. */
	control:(value:() => T, commit:(next:T) => void) => JSX.Element;
	/** Writes the setting for one monitor, or for every monitor when no label is
	 *  given. */
	set:(label:string | undefined, next:T) => void;
	/** The word `reset` knows this setting by. */
	word:string;
}) => {
	/** The panel's context: the reset command writes through it, so a row does not
	 *  need a signal of its own to reach the backend. */
	const Shared = () => useContext(Context);

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
				Shared().call(
					"reset",
					Shown() === undefined
						? { setting:properties.word }
						: { setting:properties.word, label:properties.scope },
				)
			}>
			{properties.control(Drawn, (next) => properties.set(properties.scope, next))}
		</Row>
	);
};
