//! `[workspace.package].version`, read out of a Cargo manifest.

use std::path::Path;

use anyhow::{Context, Result, anyhow};

use crate::Version::Struct::Struct as Number;

pub fn Fn(Path:&Path, Text:&str) -> Result<Number> {
	let Document = Text
		.parse::<toml_edit::DocumentMut>()
		.with_context(|| format!("{} is not valid TOML", Path.display()))?;

	let Field = Document
		.get("workspace")
		.and_then(|Workspace| Workspace.get("package"))
		.and_then(|Package| Package.get("version"))
		.and_then(|Version| Version.as_str())
		.ok_or_else(|| anyhow!("{} has no [workspace.package].version", Path.display()))?;

	Field
		.parse()
		.with_context(|| format!("{} has a version that is not semver", Path.display()))
}
