//! Recording which button opened the panel.

use tauri::tray::MouseButton;

use crate::Tray::Session::Store;

/// Record the button of the click that is being handled.
///
/// Called before the panel is toggled, not after: a right click while the panel
/// is open closes it, so the close runs inside this same click and has to see
/// this click's button rather than the one that opened the panel minutes ago.
pub fn Fn(Button:MouseButton) { *Store::Const.lock().unwrap_or_else(|Poisoned| Poisoned.into_inner()) = Some(Button); }
