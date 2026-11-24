import { currentMonitor } from "@tauri-apps/api/window";

import type { Work } from "../Type/Panel.js";

/** The work area of the monitor the panel is on.
 *
 *  `screen.availHeight` is the same fact without an IPC round trip - the webview
 *  reports the work area of the display the window is on - so a panel whose
 *  monitor cannot be read is still capped at the screen rather than left to grow
 *  past it. `undefined` is a platform that answers neither: the panel then keeps
 *  the height its content asks for, which is what it did before this cap.
 */
export default async ():Promise<Work | undefined> => {
	const monitor = await currentMonitor().catch(() => null);

	if (!monitor) {
		const Height = window.screen.availHeight;

		return Height > 0
			? { Top:0, Height:Math.floor(Height), Scale:window.devicePixelRatio || 1 }
			: undefined;
	}

	const Scale = monitor.scaleFactor;
	const Height = monitor.workArea.size.height / Scale;

	return Height > 0
		? { Top:monitor.workArea.position.y / Scale, Height:Math.floor(Height), Scale }
		: undefined;
};
