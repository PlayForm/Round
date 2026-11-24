//! The label the release before unique labels gave a monitor.

use crate::Screen::Clean;

/// The label an older build stored this monitor's overrides under.
///
/// Before a label was made unique it was the monitor's name and nothing else,
/// so two displays of the same model were stored under one key and shared one
/// set of overrides. An unnamed monitor took `monitor<index>` - the one place
/// that label was ever distinct, and the reason it was never stable: the index
/// is enumeration order, and enumeration order is not a promise.
///
/// The store on disk still holds those keys, so
/// [`Settings::Migrate::Label`](crate::Settings::Migrate::Label::Fn) reads this
/// to know where to look for a monitor's settings.
pub fn Fn(Name:Option<&str>, Index:usize) -> String {
	let Clean = Clean::Fn(Name);

	Clean.is_empty().then(|| format!("monitor{Index}")).unwrap_or(Clean)
}
