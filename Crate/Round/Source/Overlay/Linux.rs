//! Linux: `always_on_top` and `skip_taskbar` are all the window builder gives.

use tauri::{Runtime, WebviewWindow};

/// Nothing to do beyond what the window builder already asked for. A
/// compositor may still draw its own panels over the corners; there is no
/// in-process API that lifts a window above them on every desktop.
pub fn raise<R:Runtime>(_window:&WebviewWindow<R>) {}

/// No dock icon to hide.
pub fn dock(_visible:bool) {}
