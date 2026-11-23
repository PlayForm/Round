//! Building the app: the webview package first, then the Tauri bundle.

use std::path::Path;

use anyhow::Result;

use crate::{
	Build::{Args, Struct::Struct as Build},
	Process::Run,
};

/// `pnpm run Build` then `pnpm tauri build`. The Tauri CLI runs from the
/// repository root on purpose: it finds `Crate/Round/tauri.conf.json` by walking
/// down, and treats the root as the frontend directory, so
/// `beforeBuildCommand` resolves there.
pub fn Fn(Root:&Path, Option:&Build) -> Result<()> {
	Run::Fn(Root, "pnpm", &["run", "Build"])?;

	let Built = Args::Fn(Option);

	let Borrowed:Vec<&str> = Built.iter().map(String::as_str).collect();

	Run::Fn(Root, "pnpm", &Borrowed)
}
