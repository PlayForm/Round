//! Resolving a monitor into the window it gets.

use tauri::Monitor;

use crate::Screen::{Claim::Struct::Struct as Claim, Label, Legacy, Struct::Struct as Struct, Suffix};

/// The overlay window a monitor gets: its label, the two labels it has been
/// stored under before, and its bounds in the logical pixels the window builder
/// takes.
///
/// The label is the one the monitor's name asks for, which a second display of
/// the same model asks for as well - so a launch resolves the whole set through
/// [`All`](crate::Screen::All::Fn), which is where the label is made unique.
/// `Qualified` is that same claim with the monitor's position on the desktop
/// appended, which is the label it carries whenever it is not alone; the bounds
/// and the two former labels are the same either way.
pub fn Fn(Monitor:&Monitor, Index:usize) -> Struct {
	let Factor = Monitor.scale_factor();

	let Size = Monitor.size().to_logical::<i32>(Factor);

	let Position = Monitor.position().to_logical::<i32>(Factor);

	let Origin = Monitor.position();

	let Name = Monitor.name().map(String::as_str);

	let Claim = Claim { Label:Label::Fn(Name), X:Origin.x, Y:Origin.y };

	Struct {
		Label:Claim.Label.clone(),
		Qualified:Suffix::Fn(&Claim),
		Legacy:Legacy::Fn(Name, Index),
		Width:Size.width,
		Height:Size.height,
		X:Position.x,
		Y:Position.y,
	}
}
