import { LogicalPosition, LogicalSize } from "@tauri-apps/api/dpi";
import type { getCurrentWebviewWindow } from "@tauri-apps/api/webviewWindow";

import Screen from "./Screen.js";

/** The width the panel is drawn at, in logical pixels - `Menu::Width`. */
const Width = 320;

/** The transparent margin the window keeps around the panel, on each side. */
const Margin = 6;

/** Logical pixels of the work area kept clear of the panel: the gap it hangs
 *  below the tray icon by, plus the margin above and below it. A panel at the cap
 *  therefore ends 6px short of the work area's edge rather than on it. */
const Reserve = 6 + 2 * Margin;

/** Size the window to the panel, and keep the panel inside the work area.
 *
 *  Rust places the panel from the size it last had, which is not the size it is
 *  about to be drawn at: the first open after an update, and any open after a
 *  monitor was added, grow the panel after it has been placed. So the panel also
 *  pulls itself back inside the work area - a panel that grew under a tray icon
 *  with room only for the old height would otherwise hang past the bottom edge
 *  with its Quit button off screen.
 *
 *  Every step here is a nicety: a panel that could not be measured or moved is
 *  still a panel, so no failure aborts the render.
 */
export default async (Element:HTMLDivElement, Window:ReturnType<typeof getCurrentWebviewWindow>) => {
	const Area = await Screen();

	// The cap goes on before the measurement, not after it: the panel scrolls
	// inside the window, so the height it measures is the height it is drawn at.
	Element.style.maxHeight = Area ? `${Math.max(Area.Height - Reserve, 2 * Margin)}px` : "";

	const Height = Math.ceil(Element.getBoundingClientRect().height) + 2 * Margin;

	await Window.setSize(new LogicalSize(Width, Height)).catch((error) =>
		console.warn("round: the panel could not be resized", error),
	);

	if (!Area) return;

	const Position = await Window.outerPosition().catch(() => null);

	if (!Position) return;

	const Y = Position.y / Area.Scale;
	const Top = Math.min(Math.max(Y, Area.Top), Math.max(Area.Top, Area.Top + Area.Height - Height));

	if (Math.abs(Top - Y) < 1) return;

	await Window.setPosition(new LogicalPosition(Position.x / Area.Scale, Top)).catch((error) =>
		console.warn("round: the panel could not be moved back on screen", error),
	);
};
