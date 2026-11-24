//! Qualifying a label two monitors both asked for.

use crate::Screen::Claim::Struct::Struct as Claim;

/// `Claim`'s label, qualified with the monitor's own position on the desktop.
///
/// The position is unique among displays that are not mirrored, and it is a fact
/// about the display rather than about the order the system enumerated the
/// monitors in - which is what makes the label survive a relaunch. A negative
/// origin keeps its sign; `-` is one of the few characters Tauri accepts besides
/// alphanumerics, along with `_`.
pub fn Fn(Claim:&Claim) -> String { format!("{}_{}x{}", Claim.Label, Claim.X, Claim.Y) }
