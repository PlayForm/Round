//! The gates CI runs, run locally in the same order.

use std::path::Path;

use anyhow::Result;

use crate::Process::Run;

/// `fmt`, then `clippy -D warnings`, then `check` - the order CI uses, so the
/// first failure is the same failure.
pub fn Fn(Root:&Path) -> Result<()> {
	Run::Fn(Root, "cargo", &["fmt", "--all", "--", "--check"])?;

	Run::Fn(Root, "cargo", &["clippy", "--workspace", "--all-targets", "--", "-D", "warnings"])?;

	Run::Fn(Root, "cargo", &["check", "--workspace", "--all-targets"])
}
