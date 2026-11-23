//! The whole ceremony, as the list of commands it is.

use std::path::Path;

use crate::{
	Manifest::{Entry, Relative},
	Release::{
		Step::{New, Struct::Struct as Step},
		Struct::Struct as Release,
	},
	Version::Tag,
};

/// Every command in order, as data - so `--dry-run` prints exactly what a real
/// run would execute instead of a prose description of it.
pub fn Fn(Root:&Path, Option:&Release, Branch:&str, Remote:&str) -> Vec<Step> {
	let Tag = Tag::Fn(&Option.Version);

	let Add:Vec<String> = ["add".to_owned(), "CHANGELOG.md".to_owned()]
		.into_iter()
		.chain(Entry::Fn(Root).iter().map(|Manifest| Relative::Fn(Root, Manifest)))
		.collect();

	let Add:Vec<&str> = Add.iter().map(String::as_str).collect();

	let mut Steps = vec![
		New::Fn("git", &Add),
		New::Fn("git", &["commit", "-m", &format!("chore(release): {}", Option.Version)]),
		New::Fn("git", &["tag", "-a", &Tag, "-m", &Tag]),
		New::Fn("git", &["push", Remote, Branch]),
		New::Fn("git", &["push", Remote, &Tag]),
	];

	Steps.extend(Option.Publish.then(|| {
		New::Fn(
			"gh",
			&["workflow", "run", "Publish.yml", "--ref", Branch, "-f", "publish_crates=true"],
		)
	}));

	Steps
}
