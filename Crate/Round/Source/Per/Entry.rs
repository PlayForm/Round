//! One monitor's own value, when it has one.

use crate::Per::Struct::Struct;

/// `Label`'s own value, or `None` when it follows the default.
///
/// [`Of`](crate::Per::Of::Fn) is this resolved against the default. The panel
/// needs the unresolved answer too, because it marks a row whose monitor carries
/// an override, and a value that happens to equal the default cannot say whether
/// the monitor chose it.
pub fn Fn<T:Copy>(State:&Struct<T>, Label:&str) -> Option<T> { State.Monitor.get(Label).copied() }
