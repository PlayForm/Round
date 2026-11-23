//! The label of the scrim over one monitor.

use crate::Backdrop::Prefix;

/// The label of the scrim over the monitor at `Index`.
///
/// Indexed rather than named after the monitor: a window is built once and
/// re-placed on every open, so nothing depends on which display it covered the
/// time it was created, and a display that is plugged back in under another
/// name still lands on a window that already exists.
pub fn Fn(Index:usize) -> String { format!("{}{Index}", Prefix::Const) }
