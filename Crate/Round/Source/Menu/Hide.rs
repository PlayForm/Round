//! Close the panel and the scrims together.

use tauri::{AppHandle, Runtime, WebviewWindow};

use crate::Backdrop::Close as BackdropClose;

/// Close the panel and the scrims together.
///
/// Idempotent: hiding a window that is already hidden is a no-op, so a second
/// `dismiss`, or one that arrives after the tray already closed the panel, is
/// harmless. The scrims are closed first: a scrim without a panel over it is a
/// window that eats clicks for no reason, so it must never be the one left
/// behind.
pub fn Fn<R:Runtime>(App:&AppHandle<R>, Window:&WebviewWindow<R>) -> tauri::Result<()> {
	BackdropClose::Fn(App);

	Window.hide()
}
