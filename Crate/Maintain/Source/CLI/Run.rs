//! Parsing the command line and routing it to the ceremony it names.

use anyhow::Result;

use clap::Parser;

use crate::{
	Build,
	Check,
	CLI::{Enum::Enum as Command, Publish, Struct::Struct as Cli, Version},
	Release,
	Root::Find,
};

pub fn Fn() -> Result<()> {
	let Surface = Cli::parse();

	let Root = Find::Fn()?;

	match Surface.Command {
		Command::Version { Action } => Version::Run::Fn(&Root, Action),
		Command::Check => Check::Run::Fn(&Root),
		Command::Build { Debug, Bundles } => Build::Run::Fn(&Root, &crate::Build::Struct::Struct { Debug, Bundles }),
		Command::Release { Version, Publish, DryRun, AllowDirty } => {
			Release::Run::Fn(&Root, &crate::Release::Struct::Struct { Version, Publish, DryRun, AllowDirty })
		}
		Command::Publish { DryRun } => Publish::Fn(&Root, DryRun),
	}
}
