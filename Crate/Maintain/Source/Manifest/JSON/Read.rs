//! The top-level `version` of a JSON manifest.

use std::path::Path;

use anyhow::{Context, Result, anyhow};

use crate::Version::Struct::Struct as Number;

/// `None` means the file carries no `version` key at all. A key that is there
/// and is not a string is a broken manifest, not an absent one.
pub fn Fn(Path:&Path, Text:&str) -> Result<Option<Number>> {
	let Parsed:serde_json::Value =
		serde_json::from_str(Text).with_context(|| format!("{} is not valid JSON", Path.display()))?;

	match Parsed.get("version") {
		None => Ok(None),
		Some(serde_json::Value::String(Field)) => Field
			.parse()
			.with_context(|| format!("{} has a version that is not semver", Path.display()))
			.map(Some),
		Some(_) => Err(anyhow!("{} has no string `version`", Path.display())),
	}
}
