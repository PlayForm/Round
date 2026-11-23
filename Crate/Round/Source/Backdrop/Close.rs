//! Hiding every scrim.

use tauri::{AppHandle, Manager, Runtime};

use crate::Backdrop::Owns;

/// Hide every scrim.
///
/// Idempotent: a window that is already hidden is left hidden, so closing twice
/// costs nothing and cannot fail - the scrim's second click, or a `dismiss` that
/// arrives after the tray already closed the panel.
pub fn Fn<R:Runtime>(App:&AppHandle<R>) {
	App.webview_windows()
		.into_values()
		.filter(|Window| Owns::Fn(Window.label()))
		.for_each(|Window| {
			let _ = Window.hide();
		});
}
