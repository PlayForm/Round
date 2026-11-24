//! One monitor's overrides, taken from the label they were stored under.

use std::collections::BTreeMap;

use crate::Screen::Struct::Struct as Screen;

/// The pair that gives this screen its former label's value under the label it
/// carries now, or `None` when there is nothing to carry.
///
/// Nothing to carry is three cases and one answer: the monitor carries the label
/// it was stored under, the label it carries now already holds a value of its
/// own, or the store never held the former one. The caller merges the pairs only
/// after every monitor has been asked, which is what lets two displays that
/// shared a label both keep the value they were drawn with.
pub fn Fn<T:Clone>(Monitor:&BTreeMap<String, T>, Screen:&Screen) -> Option<(String, T)> {
	match Monitor.get(&Screen.Label) {
		Some(_) => None,
		None => Monitor.get(&Screen.Legacy).map(|Value| (Screen.Label.clone(), Value.clone())),
	}
}
