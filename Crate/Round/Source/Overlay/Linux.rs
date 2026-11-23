//! Linux: `always_on_top` and `skip_taskbar` are all the window builder gives.

use tauri::{Runtime, WebviewWindow};

/// Nothing to do beyond what the window builder already asked for. A
/// compositor may still draw its own panels over the corners; there is no
/// in-process API that lifts a window above them on every desktop.
pub fn art<R:Runtime>(_window:&WebviewWindow<R>) {}

/// Put one scrim above the windows below it.
///
/// There is no per-layer level to ask for here, so the ordering is the raise:
/// `set_always_on_top(true)` lifts the window back over the rest, and the panel
/// does the same after the scrims are up. A compositor that refuses to stack a
/// client above another leaves the scrim under the panel, where a click on the
/// panel is still the panel's.
pub fn scrim<R:Runtime>(window:&WebviewWindow<R>) { let _ = window.set_always_on_top(true); }

/// Put the panel above every scrim. The raise is what orders it, and it runs
/// after the scrims are shown.
pub fn panel<R:Runtime>(window:&WebviewWindow<R>) { let _ = window.set_always_on_top(true); }

/// Put a window on screen. The portable call is all the builder leaves to do;
/// whether the compositor also gives it the keyboard is not something this
/// module can refuse.
pub fn front<R:Runtime>(window:&WebviewWindow<R>) -> tauri::Result<()> { window.show() }

/// No dock icon to hide.
pub fn dock(_visible:bool) {}
