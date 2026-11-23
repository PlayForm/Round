//! Dispatching the crates.io publish.

use std::path::Path;

use anyhow::Result;

use crate::Process;

/// `gh workflow run Publish.yml --ref <branch> -f publish_crates=true`. A tag
/// push never publishes, so this dispatch is the only way to reach crates.io.
pub fn Fn(Root:&Path, DryRun:bool) -> Result<()> {
	let Branch = Process::Capture::Fn(Root, "git", &["rev-parse", "--abbrev-ref", "HEAD"])?;

	let Args = ["workflow", "run", "Publish.yml", "--ref", Branch.as_str(), "-f", "publish_crates=true"];

	DryRun
		.then(|| println!("$ gh {}", Args.join(" ")))
		.map_or_else(|| Process::Run::Fn(Root, "gh", &Args), |()| Ok(()))
}
