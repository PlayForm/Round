import { render } from "solid-js/web";

import "./Asset/CSS/Menu.css";

import Panel from "./Element/Panel.jsx";

/** The panel's entry: one component, mounted into the document the window loads.
 *
 *  Everything the panel is lives under `Element/` and `Fn/`: one anonymous default
 *  export per element, one per function, with the types in `Type/Panel` and the
 *  shared state in `Context`. This file only mounts the root. */
render(() => <Panel />, document.getElementById("menu") as HTMLElement);
