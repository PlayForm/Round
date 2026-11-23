//! Finding the repository, so every command works from any subdirectory.

use std::path::PathBuf;

use anyhow::{Context, Result};

use crate::Process::Capture;

/// The repository root: the git toplevel when there is one and it holds the
/// workspace manifest, the working directory otherwise.
pub fn Fn() -> Result<PathBuf> {
	let Current = std::env::current_dir().context("the working directory could not be read")?;

	let Top = Capture::Fn(&Current, "git", &["rev-parse", "--show-toplevel"])
		.ok()
		.map(PathBuf::from)
		.filter(|Path| Path.join("Cargo.toml").is_file());

	Ok(Top.unwrap_or(Current))
}
