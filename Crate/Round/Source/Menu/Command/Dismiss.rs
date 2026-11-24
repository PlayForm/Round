//! Closing the panel from the scrim.

use tauri::{AppHandle, Manager, Runtime};

use crate::{
	Backdrop::Close as BackdropClose,
	Menu::{Hide, Label},
};

/// Close the panel and the scrims.
///
/// This is the scrim's only command, and the only path that dismisses the panel
/// on its own: a pointer-down delivered to a window of this app that is not the
/// panel. Nothing about it reads focus, activation, timing or geometry.
#[tauri::command(rename = "dismiss")]
pub fn Dismiss<R:Runtime>(App:AppHandle<R>) {
	match App.get_webview_window(Label::Const) {
		Some(Window) => {
			let _ =
				Hide::Fn(&App, &Window).inspect_err(|Error| eprintln!("round: the panel could not be closed: {Error}"));
		}
		// A scrim outliving its panel is the one state worse than an open panel,
		// so a dismissal still closes the scrims when there is no panel window
		// left to hide.
		None => BackdropClose::Fn(&App),
	}
}
