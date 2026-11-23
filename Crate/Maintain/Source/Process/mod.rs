//! Running the tools, with the transcript a human needs when it fails.

use std::{path::Path, process::Command};

use crate::Error::{Error, Result};

/// Run a command, inherit its output, and fail when it does.
pub fn run(root:&Path, program:&str, args:&[&str]) -> Result<()> {
	println!("$ {program} {}", args.join(" "));

	let status = Command::new(program)
		.args(args)
		.current_dir(root)
		.status()
		.map_err(|source| Error::Command(format!("`{program}` could not be started: {source}")))?;

	status
		.success()
		.then_some(())
		.ok_or_else(|| Error::Command(format!("`{program} {}` exited with {status}", args.join(" "))))
}

/// Run a command and return its trimmed stdout.
pub fn capture(root:&Path, program:&str, args:&[&str]) -> Result<String> {
	let output = Command::new(program)
		.args(args)
		.current_dir(root)
		.output()
		.map_err(|source| Error::Command(format!("`{program}` could not be started: {source}")))?;

	output
		.status
		.success()
		.then(|| String::from_utf8_lossy(&output.stdout).trim().to_owned())
		.ok_or_else(|| Error::Command(format!("`{program} {}` failed", args.join(" "))))
}
