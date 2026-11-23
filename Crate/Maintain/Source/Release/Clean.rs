//! Refusing a tree that has work in it.

use std::path::Path;

use anyhow::{Result, anyhow};

use crate::Process::Capture;

/// The tag always names a commit whose content is exactly what was verified, so
/// uncommitted work has to be either committed or waved through by name.
pub fn Fn(Root:&Path) -> Result<()> {
	let Dirty = Capture::Fn(Root, "git", &["status", "--porcelain"])?;

	Dirty.is_empty().then_some(()).ok_or_else(|| {
		anyhow!(
			"{} file(s) uncommitted; commit them or pass --allow-dirty",
			Dirty.lines().count()
		)
	})
}
