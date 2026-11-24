//! Settling which monitor gets which label.

use crate::Screen::{Claim::Struct::Struct as Claim, Suffix, Tally};

/// One label per claim, unique within the set and in the order the claims
/// arrived.
///
/// A claim no other monitor made is already a label, and is left exactly as it
/// is: the store on disk is keyed by it, so an untouched monitor keeps its
/// settings across the launch that makes labels unique. A claim two or more
/// monitors made is qualified by [`Suffix`], which is where the position of the
/// monitor settles it.
///
/// Two monitors can still agree after that - the same name in the same place is
/// two displays showing one picture, and nothing they report tells them apart.
/// There the enumeration index is the last resort: it is not stable, and a pair
/// that draws the same picture swaps nothing visible by swapping labels.
pub fn Fn(Claims:&[Claim]) -> Vec<String> {
	let Shared = Tally::Fn(Claims.iter().map(|Claim| Claim.Label.as_str()));

	let Qualified:Vec<String> = Claims
		.iter()
		.map(|Claim| match Shared.get(Claim.Label.as_str()) {
			Some(&1) => Claim.Label.clone(),
			_ => Suffix::Fn(Claim),
		})
		.collect();

	let Taken = Tally::Fn(Qualified.iter().map(String::as_str));

	Qualified
		.into_iter()
		.enumerate()
		.map(|(Index, Label)| match Taken.get(Label.as_str()) {
			Some(&1) => Label,
			_ => format!("{Label}_{Index}"),
		})
		.collect()
}
