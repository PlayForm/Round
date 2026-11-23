/**
 * @module Settings
 */

/** The two inks a corner can be drawn in - the words the Rust `Mode` enum serialises to. */
export type Mode = "dark" | "light";

/** What the Rust side injects as `window.settings` before the first paint. */
export type Settings = {
	size: number;

	mode: Mode;
};

/** `listen("size", ...)` payload: `Message::Size` is externally tagged. */
export type SizeMessage = {
	message: { Size: number };
};

/** `listen("mode", ...)` payload: `Message::Mode` is externally tagged. */
export type ModeMessage = {
	message: { Mode: Mode };
};

declare global {
	interface Window {
		settings: Settings;
	}
}
