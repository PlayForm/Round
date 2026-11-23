//! The remote the release pushes to.

use std::path::Path;

use anyhow::{Result, anyhow};

use crate::Process::Capture;

/// This repository names its remote `Source`; a clone that only has `origin`
/// still works.
pub fn Fn(Root:&Path) -> Result<String> {
	let Listed = Capture::Fn(Root, "git", &["remote"])?;

	let Names:Vec<&str> = Listed.lines().map(str::trim).filter(|Name| !Name.is_empty()).collect();

	Names
		.iter()
		.find(|Name| **Name == "Source")
		.or_else(|| Names.first())
		.map(|Name| (*Name).to_owned())
		.ok_or_else(|| anyhow!("this repository has no git remote"))
}
