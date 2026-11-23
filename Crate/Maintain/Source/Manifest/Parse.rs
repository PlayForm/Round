//! The version a manifest holds, taken from text already read.

use anyhow::{Result, anyhow};

use crate::{
	Manifest::{Cargo, Enum::Enum as Kind, JSON, Struct::Struct as Manifest},
	Version::Struct::Struct as Number,
};

/// Absence is only an answer for the Tauri config: a `package.json` without a
/// `version` is a broken manifest, and so is one whose `version` is not a
/// string. Text is taken rather than a path so the writer can read the file
/// once and still ask the same question.
pub fn Fn(Manifest:&Manifest, Text:&str) -> Result<Option<Number>> {
	match Manifest.Kind {
		Kind::Cargo => Cargo::Read::Fn(&Manifest.Path, Text).map(Some),
		Kind::JSON => JSON::Read::Fn(&Manifest.Path, Text)?
			.map(Some)
			.ok_or_else(|| anyhow!("{} has no string `version`", Manifest.Path.display())),
		Kind::Tauri => JSON::Read::Fn(&Manifest.Path, Text),
	}
}
