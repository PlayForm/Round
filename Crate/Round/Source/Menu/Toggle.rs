//! Opening the panel, or closing it when it is already open.

use tauri::{AppHandle, Manager, Rect, Runtime};

use crate::Menu::{Build, Hide, Label, Open};

/// Show the panel under `Anchor`, or hide it when it is already open.
///
/// `Anchor` is the tray icon's rect: the panel opens under the icon, on
/// whichever screen that icon lives on. The icon is outside the scrim - the
/// scrims stop at each monitor's work area - so this click is delivered to the
/// menu bar rather than swallowed, which is what keeps the tray usable while the
/// panel is open.
pub fn Fn<R:Runtime>(App:&AppHandle<R>, Anchor:Rect) -> tauri::Result<()> {
	let Window = match App.get_webview_window(Label::Const) {
		Some(Window) => Window,
		None => Build::Fn(App)?,
	};

	match Window.is_visible().unwrap_or(false) {
		true => Hide::Fn(App, &Window),
		false => Open::Fn(App, &Window, Anchor),
	}
}
