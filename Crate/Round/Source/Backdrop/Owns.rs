//! Telling a scrim window apart from an art window.

use crate::Backdrop::Prefix;

/// Whether `Label` names a scrim window.
///
/// The overlay broadcast asks this before treating a window as art: the scrim is
/// hidden and shown by the panel, and a broadcast that "restored" it would leave
/// a click-eating window over the desktop with no panel on it. The test is
/// `backdrop` followed by digits only, so it cannot claim an art window whose
/// label merely starts with the same letters.
pub fn Fn(Label:&str) -> bool {
	Label
		.strip_prefix(Prefix::Const)
		.is_some_and(|Index| !Index.is_empty() && Index.bytes().all(|Byte| Byte.is_ascii_digit()))
}
