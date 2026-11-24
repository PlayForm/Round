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
		const height = window.screen.availHeight;

		return height > 0 ? { top:0, height:Math.floor(height), scale:window.devicePixelRatio || 1 } : undefined;
	}

	const scale = monitor.scaleFactor;
	const height = monitor.workArea.size.height / scale;

	return height > 0 ? { top:monitor.workArea.position.y / scale, height:Math.floor(height), scale } : undefined;
};
