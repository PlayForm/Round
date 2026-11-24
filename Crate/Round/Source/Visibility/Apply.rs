//! Making one overlay window's visibility match a value.

use tauri::{Runtime, WebviewWindow};

use crate::{
	Overlay::{Layer::Trait::Trait as _, Platform::Struct as Platform},
	Visibility::Enum::Enum as Visibility,
};

/// Show or hide one overlay window to match `Value`.
///
/// A window already in the wanted state is left untouched - not as an
/// optimisation, but because showing one is not a no-op: it re-orders the
/// window, and on macOS `show()` would make it key and take the keyboard from
/// the panel. Every command ends in a broadcast, so re-showing an overlay that
/// is already up would re-order the desktop on every slider step.
pub fn Fn<R:Runtime>(Value:Visibility, Window:&WebviewWindow<R>) -> tauri::Result<()> {
	match (Value, Window.is_visible().unwrap_or(false)) {
		// Wanted up and up, wanted down and down: nothing to do.
		(Visibility::Shown, true) | (Visibility::Hidden, false) => Ok(()),
		(Visibility::Shown, false) => Platform::Front(Window),
		(Visibility::Hidden, true) => Window.hide(),
	}
}
