import { invoke } from "@tauri-apps/api/core";

import "./Asset/CSS/Backdrop.css";

/**
 * The scrim's whole behaviour.
 *
 * A `pointerdown` on this document is the one event that states its own cause:
 * the pointer went down on the backdrop, which is to say outside the panel.
 * Every button counts - a right or middle click is a click that left the panel
 * too - and the listener runs in the capture phase, so nothing this document
 * grows later can swallow the event before it arrives.
 *
 * The command is idempotent, so a second pointerdown that reaches a scrim the
 * first one already hid costs nothing.
 */
addEventListener(
	"pointerdown",
	() => {
		void invoke("dismiss").catch((error) => console.error("dismiss failed", error));
	},
	{ capture: true },
);

/** A scrim has no menu to open, so the system one is refused. */
addEventListener("contextmenu", (event) => event.preventDefault());
