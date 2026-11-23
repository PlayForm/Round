//! Running a tool, with the transcript a human needs when it fails.

use std::{path::Path, process::Command};

use anyhow::{Context, Result, anyhow};

/// Run a command, inherit its output, and fail when it does. The `$` line is
/// printed before the tool starts, so a failure leaves the command that failed
/// in the transcript above it.
pub fn Fn(Root:&Path, Program:&str, Args:&[&str]) -> Result<()> {
	println!("$ {Program} {}", Args.join(" "));

	let Status = Command::new(Program)
		.args(Args)
		.current_dir(Root)
		.status()
		.with_context(|| format!("`{Program}` could not be started"))?;

	Status
		.success()
		.then_some(())
		.ok_or_else(|| anyhow!("`{Program} {}` exited with {Status}", Args.join(" ")))
}
