//! Cutting a release: bump, commit, tag, push, and optionally publish.

use std::path::Path;

use anyhow::{Result, anyhow};

use crate::{
	Manifest::Current,
	Process::Capture,
	Release::{
		Clean,
		Mode::{Enum::Enum as Mode, Of},
		Perform,
		Preview,
		Remote,
		Steps,
		Struct::Struct as Release,
	},
};

pub fn Fn(Root:&Path, Option:&Release) -> Result<()> {
	let Current = Current::Fn(Root)?;

	// Tags and crates.io versions are append-only: a release that does not move
	// forward can only fail later, after the tag exists.
	(Option.Version > Current)
		.then_some(())
		.ok_or_else(|| anyhow!("{} is not after the current {Current}", Option.Version))?;

	Option.AllowDirty.then_some(()).map_or_else(|| Clean::Fn(Root), Ok)?;

	let Branch = Capture::Fn(Root, "git", &["rev-parse", "--abbrev-ref", "HEAD"])?;

	let Remote = Remote::Fn(Root)?;

	let Steps = Steps::Fn(Root, Option, &Branch, &Remote);

	match Of::Fn(Option.DryRun) {
		Mode::Preview => Preview::Fn(Option, &Steps),
		Mode::Perform => Perform::Fn(Root, Option, &Steps),
	}
}
