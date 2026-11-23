//! Running a tool for its answer rather than its transcript.

use std::{path::Path, process::Command};

use anyhow::{Context, Result, anyhow};

/// Run a command and return its trimmed stdout. Nothing is printed: this is the
/// shape the ceremony uses to ask git a question.
pub fn Fn(Root:&Path, Program:&str, Args:&[&str]) -> Result<String> {
	let Output = Command::new(Program)
		.args(Args)
		.current_dir(Root)
		.output()
		.with_context(|| format!("`{Program}` could not be started"))?;

	Output
		.status
		.success()
		.then(|| String::from_utf8_lossy(&Output.stdout).trim().to_owned())
		.ok_or_else(|| anyhow!("`{Program} {}` failed", Args.join(" ")))
}
