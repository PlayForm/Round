//! Writing a version into a manifest, leaving the rest of the file alone.

use std::fs;

use anyhow::{Context, Result};

use crate::{
	Manifest::{Cargo, Enum::Enum as Kind, JSON, Parse, Struct::Struct as Manifest},
	Version::Struct::Struct as Number,
};

pub fn Fn(Manifest:&Manifest, Version:&Number) -> Result<()> {
	let Text =
		fs::read_to_string(&Manifest.Path).with_context(|| format!("{} could not be read", Manifest.Path.display()))?;

	let Held = Parse::Fn(Manifest, &Text)?;

	let Written = match Manifest.Kind {
		Kind::Cargo => Cargo::Write::Fn(&Manifest.Path, &Text, Version)?,
		Kind::JSON | Kind::Tauri => Held.map_or_else(
			|| Text.clone(),
			|Held| JSON::Replace::Fn(&Text, &Held.to_string(), &Version.to_string()).unwrap_or_else(|| Text.clone()),
		),
	};

	fs::write(&Manifest.Path, Written).with_context(|| format!("{} could not be written", Manifest.Path.display()))
}
