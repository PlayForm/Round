//! The version command's answer: the version alone on stdout, the table on stderr.

use std::path::Path;

use anyhow::{Result, anyhow};

use crate::{
	Manifest::{Current, Entry, Read, Relative},
	Version::Struct::Struct as Number,
};

/// The table is for a human and goes to stderr; stdout carries the version
/// alone, so `$(Maintain version)` is the version. A manifest that disagrees is
/// the failure this command exists for, and it is named in the message.
pub fn Fn(Root:&Path) -> Result<()> {
	let Current = Current::Fn(Root)?;

	let Rows:Vec<(String, Option<Number>)> = Entry::Fn(Root)
		.into_iter()
		.map(|Manifest| Read::Fn(&Manifest).map(|Held| (Relative::Fn(Root, &Manifest), Held)))
		.collect::<Result<_>>()?;

	let Shown:Vec<(String, String)> = Rows
		.iter()
		.map(|(File, Held)| {
			let Value = Held
				.as_ref()
				.map(ToString::to_string)
				.unwrap_or_else(|| format!("inherits {Current}"));

			(File.clone(), Value)
		})
		.collect();

	let Width = Shown.iter().map(|(File, _)| File.len()).max().unwrap_or(0);

	Shown.iter().for_each(|(File, Value)| eprintln!("{File:Width$}  {Value}"));

	let Stale:Vec<&String> = Rows
		.iter()
		.filter_map(|(File, Held)| Held.as_ref().filter(|Held| **Held != Current).map(|_| File))
		.collect();

	Stale.is_empty().then(|| println!("{Current}")).ok_or_else(|| {
		anyhow!(
			"{} manifest(s) disagree with Cargo.toml ({Current}): run `Maintain version sync`",
			Stale.len()
		)
	})
}
