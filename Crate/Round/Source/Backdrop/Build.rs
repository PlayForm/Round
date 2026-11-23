//! Building one scrim window.

use tauri::{AppHandle, Runtime, WebviewUrl, WebviewWindow, WebviewWindowBuilder};

use crate::Window::Chrome;

/// The scrim window, built hidden and re-used afterwards.
///
/// Not focusable, so a click on the scrim cannot take the keyboard from the
/// panel or from the app the user was typing in; `accept_first_mouse` then makes
/// that same click count even when Round is inactive, so the dismissal is the
/// click the user made rather than the one after it.
pub fn Fn<R:Runtime>(App:&AppHandle<R>, Label:&str) -> tauri::Result<WebviewWindow<R>> {
	Chrome::Fn(
		WebviewWindowBuilder::new(App, Label, WebviewUrl::App("backdrop.html".into())),
		"",
	)
	.shadow(false)
	.focusable(false)
	.accept_first_mouse(true)
	.focused(false)
	.disable_drag_drop_handler()
	.build()
}
