//! The subcommands.

use clap::Subcommand;

use crate::{
	CLI::Version::Enum::Enum as Action,
	Version::Struct::Struct as Number,
};

#[derive(Subcommand)]
pub enum Enum {
	/// Show the version, and fail when a manifest disagrees with Cargo.toml.
	Version {
		#[command(subcommand)]
		Action:Option<Action>,
	},
	/// Run the gates CI runs: fmt, clippy -D warnings, check.
	Check,
	/// Build the webview package and the Tauri bundle.
	Build {
		#[arg(long = "debug")]
		Debug:bool,
		#[arg(long = "bundles")]
		Bundles:Option<String>,
	},
	/// Bump, commit, tag and push a release.
	Release {
		Version:Number,
		#[arg(long = "publish")]
		Publish:bool,
		#[arg(long = "dry-run")]
		DryRun:bool,
		#[arg(long = "allow-dirty")]
		AllowDirty:bool,
	},
	/// Dispatch Publish.yml with publish_crates=true.
	Publish {
		#[arg(long = "dry-run")]
		DryRun:bool,
	},
}
