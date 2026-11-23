//! A window's own outer size.

use tauri::{Runtime, WebviewWindow};

use crate::Bounds::Struct::Struct;

/// The physical size `Window` currently occupies, or `None` before the window
/// has a size of its own.
///
/// `None` rather than a default: the caller knows what it asked the window to
/// be, and a fallback chosen here would be a second opinion about that.
pub fn Fn<R:Runtime>(Window:&WebviewWindow<R>) -> Option<Struct> {
	Window.outer_size().ok().map(|Size| Struct {
		X:0.0,
		Y:0.0,
		Width:f64::from(Size.width),
		Height:f64::from(Size.height),
	})
}
