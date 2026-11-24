import type { State } from "../Type/Panel.js";

import Size from "./Size.js";

/** The scope in full, spelled out under the selector.
 *
 *  The closed select shows one truncated line, and the part it truncates first is
 *  the name: two displays both reading "Monitor #26..." is exactly the state the
 *  selector has to be usable in. So the panel repeats the selection here, where it
 *  can wrap, with the facts that tell the entries apart. */
export default (state:State, label:string | undefined) => {
	const index = label === undefined ? -1 : state.monitors.findIndex((monitor) => monitor.label === label);

	const monitor = index < 0 ? undefined : state.monitors[index];

	return monitor === undefined
		? `All monitors · ${state.monitors.length} connected`
		: `${index + 1} of ${state.monitors.length} · ${monitor.name} · ${Size(monitor)}`;
};
