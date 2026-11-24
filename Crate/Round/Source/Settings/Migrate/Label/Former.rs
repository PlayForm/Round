//! The labels a display's overrides may have been stored under.

use crate::Screen::Struct::Struct as Screen;

/// Every label this display's overrides could be under, most current first.
///
/// Three, in the order they became true: the label it carries now, the label it
/// carries while another display shares its name, and the label the release
/// before unique labels used - its name alone.
///
/// The middle one is why unplugging a twin costs nothing: the survivor goes back
/// to its bare name, and the override it was drawn with is still under the
/// qualified label it held while the twin was there. The last one is why this
/// build's first launch costs nothing either: every override written before
/// labels were unique is under the bare name.
///
/// The list is de-duplicated with its order kept. A display whose name no other
/// display shares answers to its name and to nothing else, so two of these are
/// the same string; a reader that took either would be reading one key twice,
/// and a writer that retired the wrong one would retire the label in use.
pub fn Fn(Screen:&Screen) -> Vec<String> {
	[&Screen.Label, &Screen.Qualified, &Screen.Legacy].into_iter().fold(
		Vec::new(),
		|Former:Vec<String>, Label:&String| match Former.contains(Label) {
			true => Former,
			false => Former.into_iter().chain(Some(Label.clone())).collect(),
		},
	)
}
