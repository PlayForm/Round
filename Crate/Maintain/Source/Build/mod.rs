//! Building the app: the webview package first, then the Tauri bundle.

use std::path::Path;

use crate::{Error::Result, Process};

pub struct Options {
	pub debug:bool,
	pub bundles:Option<String>,
}

/// `pnpm run Build` then `pnpm tauri build`. The Tauri CLI runs from the
/// repository root on purpose: it finds `Crate/Round/tauri.conf.json` by walking
/// down, and treats the root as the frontend directory, so
/// `beforeBuildCommand` resolves there.
pub fn run(root:&Path, options:&Options) -> Result<()> {
	Process::run(root, "pnpm", &["run", "Build"])?;

	let mut args:Vec<&str> = vec!["tauri", "build"];

	if options.debug {
		args.push("--debug");
	}

	if let Some(bundles) = &options.bundles {
		args.push("--bundles");
		args.push(bundles);
	}

	Process::run(root, "pnpm", &args)
}
