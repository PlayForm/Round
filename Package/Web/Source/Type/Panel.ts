import type { Accessor } from "solid-js";

/** One setting as one monitor sees it: the value it is drawn with, and whether
 *  that value is its own. A value alone cannot say the second half - a monitor
 *  tuned to exactly the default is still an override - so Rust sends both. */
export type Value<T> = { value:T; origin:"inherit" | "override" };

/** One monitor, with every setting already resolved for it.
 *
 *  `width`/`height` are the display's own pixels, and they are the only facts
 *  here that no setting reads: a name does not identify a display - two of them
 *  routinely arrive with the same one - so the panel needs a second fact to tell
 *  the entries of the scope selector apart. */
export type Monitor = {
	label:string;
	name:string;
	width:number;
	height:number;
	radius:Value<number>;
	ink:Value<"dark" | "light">;
	hidden:Value<boolean>;
};

/** Everything the panel draws: each per-monitor setting's default, and every
 *  monitor's own answer for the three of them. The dock icon is the one control
 *  that is not per monitor, and travels once. */
export type State = {
	radius:number;
	maximum:number;
	ink:"dark" | "light";
	hidden:boolean;
	dock_hidden:boolean;
	monitors:Monitor[];
};

/** What every row in the panel shares: the Rust state, the command that writes
 *  it, and the scope the rows apply to.
 *
 *  It travels through a context rather than module scope. A module signal is a
 *  singleton with no owner - nothing can dispose it, and a component that
 *  imported it could not be mounted twice or unmounted at all - while a value in
 *  a context belongs to the component that provides it, and every consumer reads
 *  it reactively. */
export type Shared = {
	State:Accessor<State | undefined>;
	call:(command:string, args?:Record<string, unknown>) => Promise<void>;
	Selected:Accessor<string>;
	setSelected:(value:string) => void;
	Scope:() => string | undefined;
};

/** A work area, in logical pixels. */
export type Work = { Top:number; Height:number; Scale:number };
