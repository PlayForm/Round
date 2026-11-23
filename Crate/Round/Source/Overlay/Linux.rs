//! Linux: `always_on_top` and `skip_taskbar` are all the window builder gives.

use tauri::{Runtime, WebviewWindow};

/// Whether a click can reach the window.
///
/// Nothing here makes the overlay click-through - there is no portable way to
/// ask for it and the compositor decides - so the window does receive the click
/// and counts as part of the app.
pub fn clickable<R:Runtime>(_window:&WebviewWindow<R>) -> bool { true }

/// Nothing to do beyond what the window builder already asked for. A
/// compositor may still draw its own panels over the corners; there is no
/// in-process API that lifts a window above them on every desktop.
pub fn raise<R:Runtime>(_window:&WebviewWindow<R>) {}

/// Put one overlay window on screen. The portable call is all the builder
/// leaves to do; whether the compositor also gives it the keyboard is not
/// something this module can refuse, which is why the Menu panel's blur guard
/// also ignores focus that stayed inside the app.
pub fn present<R:Runtime>(window:&WebviewWindow<R>) -> tauri::Result<()> { window.show() }

/// No dock icon to hide.
pub fn dock(_visible:bool) {}
