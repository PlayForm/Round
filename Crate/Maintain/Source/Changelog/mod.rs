//! Promoting the changelog when a version is cut.
//!
//! A release renames the `Unreleased` heading to the version it is releasing and
//! leaves a fresh, empty `Unreleased` above it. Rewriting the block is the one
//! thing the ceremony must not do: what was written under `Unreleased` is the
//! record of the release, and it moves down intact.

use std::{
	fs,
	path::{Path, PathBuf},
};

use crate::{
	Error::{Error, Result},
	Version::Version,
};

/// The heading a release promotes.
const Unreleased:&str = "## Unreleased";

/// Rename `## Unreleased` to `## <version>` and open a new empty section above
/// it. Returns the file it wrote, or `None` when the changelog has no
/// `Unreleased` heading to promote (a release without one is legal).
pub fn promote(root:&Path, version:&Version) -> Result<Option<PathBuf>> {
	let path = root.join("CHANGELOG.md");

	let text = fs::read_to_string(&path).map_err(|source| Error::Read { path:path.clone(), source })?;

	// The file's own terminator decides how the rewrite rejoins it, so a CRLF
	// checkout does not come back with one LF-terminated line.
	let eol = text.contains("\r\n").then_some("\r\n").unwrap_or("\n");

	let mut lines:Vec<String> = text.lines().map(str::to_owned).collect();

	let found = lines.iter().position(|line| line.trim() == Unreleased);

	found.map_or(Ok(None), |index| {
		lines[index] = format!("## {version}");

		let opening = [
			Unreleased.to_owned(),
			String::new(),
			"_(nothing yet)_".to_owned(),
			String::new(),
			"---".to_owned(),
			String::new(),
		];

		lines.splice(index..index, opening);

		let written = format!("{}{eol}", lines.join(eol));

		fs::write(&path, written).map_err(|source| Error::Write { path:path.clone(), source })?;

		Ok(Some(path))
	})
}
