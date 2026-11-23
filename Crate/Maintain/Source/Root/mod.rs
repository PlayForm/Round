//! Finding the repository, so every command works from any subdirectory.

use std::path::PathBuf;

use crate::{Error::Result, Process};

/// The repository root: the git toplevel when there is one, the working
/// directory otherwise.
pub fn find() -> Result<PathBuf> {
	let cwd =
		std::env::current_dir().map_err(|source| crate::Error::Error::Read { path:PathBuf::from("."), source })?;

	let top = Process::capture(&cwd, "git", &["rev-parse", "--show-toplevel"])
		.ok()
		.map(PathBuf::from)
		.filter(|path| path.join("Cargo.toml").is_file());

	Ok(top.unwrap_or(cwd))
}
