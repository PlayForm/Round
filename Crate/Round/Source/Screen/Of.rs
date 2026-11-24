//! Resolving a monitor into the window it gets.

use tauri::Monitor;

use crate::Screen::{Label, Legacy, Struct::Struct};

/// The overlay window a monitor gets: its label, and its bounds in the logical
/// pixels the window builder takes.
///
/// The label is the one the monitor's name asks for, which a second display of
/// the same model asks for as well - so a launch resolves the whole set through
/// [`All`](crate::Screen::All::Fn), which is where the label is made unique. The
/// bounds and the label the store was written under are the same either way.
pub fn Fn(Monitor:&Monitor, Index:usize) -> Struct {
	let Factor = Monitor.scale_factor();

	let Size = Monitor.size().to_logical::<i32>(Factor);

	let Position = Monitor.position().to_logical::<i32>(Factor);

	Struct {
		Label:Label::Fn(Monitor.name().map(String::as_str)),
		Legacy:Legacy::Fn(Monitor.name().map(String::as_str), Index),
		Width:Size.width,
		Height:Size.height,
		X:Position.x,
		Y:Position.y,
	}
}
