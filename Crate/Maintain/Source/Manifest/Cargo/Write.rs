//! `[workspace.package].version`, written back into a Cargo manifest.

use std::path::Path;

use anyhow::{Context, Result};

use crate::Version::Struct::Struct as Number;

/// `toml_edit` keeps every other byte of the manifest - comments, ordering,
/// formatting - so the diff of a version bump is one line.
pub fn Fn(Path:&Path, Text:&str, Version:&Number) -> Result<String> {
	let mut Document = Text
		.parse::<toml_edit::DocumentMut>()
		.with_context(|| format!("{} is not valid TOML", Path.display()))?;

	Document["workspace"]["package"]["version"] = toml_edit::value(Version.to_string());

	Ok(Document.to_string())
}
