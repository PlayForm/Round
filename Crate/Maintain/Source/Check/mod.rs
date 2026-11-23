//! The gates CI runs, run locally in the same order.

use std::path::Path;

use crate::{Error::Result, Process};

/// `fmt`, then `clippy -D warnings`, then `check` - the order CI uses, so the
/// first failure is the same failure.
pub fn run(root:&Path) -> Result<()> {
	Process::run(root, "cargo", &["fmt", "--all", "--", "--check"])?;

	Process::run(
		root,
		"cargo",
		&["clippy", "--workspace", "--all-targets", "--", "-D", "warnings"],
	)?;

	Process::run(root, "cargo", &["check", "--workspace", "--all-targets"])
}
