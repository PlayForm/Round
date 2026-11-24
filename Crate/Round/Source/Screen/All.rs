//! Resolving every connected monitor in one pass.

use tauri::Monitor;

use crate::Screen::{Claim::Struct::Struct as Claim, Label, Of, Struct::Struct as Screen, Unique};

/// Every monitor, in enumeration order, each with a label no other monitor in
/// the set carries.
///
/// One monitor at a time cannot answer this. A label is the identity of a
/// monitor - it names its overlay window, keys its overrides and values its
/// entry in the panel's scope selector - so two displays that report the same
/// name would share all three: one set of settings between them, one window
/// between them (Tauri refuses a second window under a label it already holds)
/// and two selector entries that cannot be told apart. [`Of`] is the
/// single-monitor answer and cannot see the other one; this is the one a launch
/// is built from.
pub fn Fn(Monitors:&[Monitor]) -> Vec<Screen> {
	let Claims:Vec<Claim> = Monitors
		.iter()
		.map(|Monitor| Claim {
			Label:Label::Fn(Monitor.name().map(String::as_str)),
			X:Monitor.position().x,
			Y:Monitor.position().y,
		})
		.collect();

	Monitors
		.iter()
		.enumerate()
		.zip(Unique::Fn(&Claims))
		.map(|((Index, Monitor), Label)| Screen { Label, ..Of::Fn(Monitor, Index) })
		.collect()
}
