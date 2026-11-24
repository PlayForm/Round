import type { Monitor } from "../Type/Panel.js";

/** A monitor's resolution, as the panel prints it. */
export default (monitor:Monitor) => `${monitor.width} × ${monitor.height}`;
