//! One monitor's overrides, taken from a label it was stored under.

use std::collections::BTreeMap;

use crate::{Screen::Struct::Struct as Screen, Settings::Migrate::Label::Former};

/// The pair that gives this screen a former label's value under the label it
/// carries now, or `None` when there is nothing to carry.
///
/// Nothing to carry is two cases and one answer: the label it carries now
/// already holds a value of its own - which is what makes this idempotent, and
/// what stops a retired key from undoing a reset - or the store holds none of
/// the labels it has answered to ([`Former`]).
///
/// The caller merges the pairs only after every monitor has been asked, which is
/// what lets two displays that shared a label both keep the value they were
/// drawn with.
pub fn Fn<T:Clone>(Monitor:&BTreeMap<String, T>, Screen:&Screen) -> Option<(String, T)> {
	match Monitor.contains_key(&Screen.Label) {
		true => None,
		false => Former::Fn(Screen)
			.into_iter()
			.filter(|Label| *Label != Screen.Label)
			.find_map(|Label| Monitor.get(&Label).map(|Value| (Screen.Label.clone(), Value.clone()))),
	}
}
