//! Placing, sizing and showing one monitor's scrim.

use tauri::{AppHandle, Manager, Monitor, Runtime};

use crate::{
	Backdrop::{Build, Label},
	Bounds::{Monitor as Area, Position, Size},
	Overlay::{Layer::Trait::Trait as _, Platform::Struct as Platform},
};

/// Place, size and show one monitor's scrim.
///
/// The work area in physical pixels, not the builder's logical units: a scrim
/// one pixel outside its screen leaves a click that never dismisses.
pub fn Fn<R:Runtime>(App:&AppHandle<R>, Monitor:&Monitor, Index:usize) -> tauri::Result<()> {
	let Label = Label::Fn(Index);

	let Window = match App.get_webview_window(&Label) {
		Some(Window) => Window,
		None => Build::Fn(App, &Label)?,
	};

	let Area = Area::Fn(Monitor);

	Window.set_position(Position::Fn(Area.X, Area.Y))?;

	Window.set_size(Size::Fn(Area.Width, Area.Height))?;

	// Above the art, below the panel, and in the mouse's way. Applied on every
	// open, so a window that was reused carries the ladder it needs today.
	Platform::Scrim(&Window);

	Platform::Front(&Window)
}
