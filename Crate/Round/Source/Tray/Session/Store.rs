//! The click that opened the panel that is open now.

use std::sync::Mutex;

use tauri::tray::MouseButton;

/// Which button opened the panel that is open now, or `None` when none is open.
///
/// Process state, not per-window state: there is one tray icon and one panel, and
/// the close needs the answer after the click that produced it has long returned.
/// A poisoned lock is recovered rather than propagated - the value is a `Copy`
/// button written once per click, so there is no half-written state to distrust,
/// and losing it would leave exactly the stale highlight this module exists to
/// clear.
pub static Const:Mutex<Option<MouseButton>> = Mutex::new(None);
