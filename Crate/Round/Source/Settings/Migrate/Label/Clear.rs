//! Retiring the labels a display no longer answers to.

use std::collections::BTreeMap;

use crate::{Screen::Struct::Struct as Screen, Settings::Migrate::Label::Former};

/// `Monitor` without any of this screen's former labels.
///
/// The label the screen carries now is kept: it is the value this setting holds,
/// and a monitor whose name no other display shares has the same label before
/// and after. Every other label it has answered to has been offered to that
/// label by [`Adopt`](crate::Settings::Migrate::Label::Adopt::Fn) and is
/// dropped, or the next read would adopt it again and undo a reset.
///
/// A label no connected monitor answers to is not this function's to drop - it
/// belongs to a display that is not here right now.
pub fn Fn<T>(Monitor:BTreeMap<String, T>, Screen:&Screen) -> BTreeMap<String, T> {
	let Former = Former::Fn(Screen);

	Monitor
		.into_iter()
		.filter(|(Label, _)| *Label == Screen.Label || !Former.contains(Label))
		.collect()
}
