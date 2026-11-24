import type { Monitor } from "../Type/Panel.js";

import Measure from "./Measure.js";

/** How one monitor is named in the scope selector: its own name, then the index
 *  the system gave it and the pixels it draws at, because a name alone does not
 *  identify a display - two of them routinely arrive with the same one, and the
 *  select truncates whatever does not fit. */
export default (monitor:Monitor, index:number) => `${index + 1}. ${monitor.name} · ${Measure(monitor)}`;
