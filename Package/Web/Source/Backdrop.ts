import { invoke } from "@tauri-apps/api/core";
import { createRoot, onCleanup } from "solid-js";

import "./Asset/CSS/Backdrop.css";

/**
 * The scrim's whole behaviour.
 *
 * A `pointerdown` on this document is the one event that states its own cause:
 * the pointer went down on the backdrop, which is to say outside the panel. Every
 * button counts - a right or middle click is a click that left the panel too - and
 * the listener runs in the capture phase, so nothing this document grows later can
 * swallow the event before it arrives.
 *
 * The command is idempotent, so a second pointerdown that reaches a scrim the
 * first one already hid costs nothing.
 *
 * It lives in a `createRoot` rather than at module scope: the scrim has no markup,
 * but it still needs an owner, so that the listeners it adds are disposed through
 * Solid's own cleanup instead of being added and forgotten.
 */
createRoot(() => {
	const Dismiss = () => {
		void invoke("dismiss").catch((Error) => console.error("dismiss failed", Error));
	};

	/** A scrim has no menu to open, so the system one is refused. */
	const Refuse = (Event: Event) => Event.preventDefault();

	const Options = { capture:true } as const;

	addEventListener("pointerdown", Dismiss, Options);
	addEventListener("contextmenu", Refuse);

	onCleanup(() => {
		removeEventListener("pointerdown", Dismiss, Options);
		removeEventListener("contextmenu", Refuse);
	});
});
