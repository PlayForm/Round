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
		Position::Physical(Position) => (f64::from(Position.x), f64::from(Position.y)),
		Position::Logical(Position) => (Position.x * Scale, Position.y * Scale),
	};

	let (Width, Height) = match Anchor.size {
		Size::Physical(Size) => (f64::from(Size.width), f64::from(Size.height)),
		Size::Logical(Size) => (Size.width * Scale, Size.height * Scale),
	};

	Struct { X, Y, Width, Height }
}
