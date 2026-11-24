//! Counting how many monitors answer to each label.

use std::collections::BTreeMap;

/// How many of `Labels` each label occurs, keyed by the label.
///
/// The count is what tells a label only one monitor asked for from one two of
/// them did: the first is already an identity, the second has to be qualified.
pub fn Fn<'a>(Labels:impl Iterator<Item = &'a str>) -> BTreeMap<String, usize> {
	let mut Counts = BTreeMap::new();

	Labels.for_each(|Label| *Counts.entry(Label.to_owned()).or_insert(0) += 1);

	Counts
}
