import { createContext } from "solid-js";

import type { Shared } from "./Type/Panel.js";

/** The panel's shared state, provided by `Element/Panel` and read by every row. */
export default createContext<Shared>();
