//! A physical position for `set_position`.

use tauri::PhysicalPosition;

/// The whole-pixel position a [`Struct`](crate::Bounds::Struct) names.
///
/// Rounding happens here and nowhere else: tauri's own `Position` conversion
/// rounds a `f64` the same way, so a caller that kept the fractional value
/// through its arithmetic lands on the same pixel it always did.
pub fn Fn(X:f64, Y:f64) -> PhysicalPosition<i32> { PhysicalPosition::new(X.round() as i32, Y.round() as i32) }
