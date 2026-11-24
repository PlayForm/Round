//! Retiring a label no monitor answers to any more.

use std::collections::BTreeMap;

use crate::Screen::Struct::Struct as Screen;

/// `Monitor` without this screen's former label.
///
/// A label the screen still answers to is left alone: a monitor whose name was
/// already unique has the same label before and after, and the value under it is
/// the value this setting holds. Every other former label has been offered to
/// the screen's own label by
/// [`Adopt`](crate::Settings::Migrate::Label::Adopt::Fn) and is dropped, or the
/// next read would adopt it again and undo a reset.
///
/// A former label that no connected monitor answers to is not this function's
/// to drop - it belongs to a display that is not here right now.
pub fn Fn<T>(Monitor:BTreeMap<String, T>, Screen:&Screen) -> BTreeMap<String, T> {
	let Retired = (Screen.Legacy != Screen.Label).then(|| Screen.Legacy.clone());

	match Retired {
		Some(Retired) => Monitor.into_iter().filter(|(Label, _)| *Label != Retired).collect(),
		None => Monitor,
	}
}
