//! The version a manifest holds, read from disk.

use std::fs;

use anyhow::{Context, Result};

use crate::{
	Manifest::{Parse, Struct::Struct as Manifest},
	Version::Struct::Struct as Number,
};

/// The version this manifest holds. `None` only for a Tauri config that
/// deliberately carries no version.
pub fn Fn(Manifest:&Manifest) -> Result<Option<Number>> {
	let Text =
		fs::read_to_string(&Manifest.Path).with_context(|| format!("{} could not be read", Manifest.Path.display()))?;

	Parse::Fn(Manifest, &Text)
}
