//! Promoting the changelog when a version is cut.

use std::{
	fs,
	path::{Path, PathBuf},
};

use anyhow::{Context, Result};

use crate::{
	Changelog::{Opening, Unreleased},
	Version::Struct::Struct as Number,
};

/// Rename `## Unreleased` to `## <version>` and open a new empty section above
/// it. Returns the file it wrote, or `None` when the changelog has no
/// `Unreleased` heading to promote (a release without one is legal).
pub fn Fn(Root:&Path, Version:&Number) -> Result<Option<PathBuf>> {
	let Path = Root.join("CHANGELOG.md");

	let Text = fs::read_to_string(&Path).with_context(|| format!("{} could not be read", Path.display()))?;

	// The file's own terminator decides how the rewrite rejoins it, so a CRLF
	// checkout does not come back with one LF-terminated line.
	let Eol = Text.contains("\r\n").then_some("\r\n").unwrap_or("\n");

	let mut Lines:Vec<String> = Text.lines().map(str::to_owned).collect();

	let Found = Lines.iter().position(|Line| Line.trim() == Unreleased::Const);

	Found.map_or(Ok(None), |Index| {
		Lines[Index] = format!("## {Version}");

		Lines.splice(Index..Index, Opening::Fn());

		let Written = format!("{}{Eol}", Lines.join(Eol));

		fs::write(&Path, Written).with_context(|| format!("{} could not be written", Path.display()))?;

		Ok(Some(Path))
	})
}
