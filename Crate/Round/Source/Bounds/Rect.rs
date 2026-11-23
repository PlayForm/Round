//! The tray icon's rect, whichever unit the platform reported.

use tauri::{Position, Rect, Size};

use crate::Bounds::Struct::Struct;

/// The tray rect as physical pixels.
///
/// Tauri hands the icon's rect over in whichever unit the platform prefers - a
/// logical rect on macOS, a physical one on Windows - so both arms exist and
/// the scale factor is what makes them one shape.
pub fn Fn(Anchor:&Rect, Scale:f64) -> Struct {
	let (X, Y) = match Anchor.position {
		Position::Physical(Point) => (f64::from(Point.x), f64::from(Point.y)),
		Position::Logical(Point) => (Point.x * Scale, Point.y * Scale),
	};

	let (Width, Height) = match Anchor.size {
		Size::Physical(Extent) => (f64::from(Extent.width), f64::from(Extent.height)),
		Size::Logical(Extent) => (Extent.width * Scale, Extent.height * Scale),
	};

	Struct { X, Y, Width, Height }
}
