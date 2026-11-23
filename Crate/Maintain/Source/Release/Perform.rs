//! Doing what the plan says: promote, write, prove, then run the commands.

use std::path::Path;

use anyhow::{Result, anyhow};

use crate::{
	Changelog::Promote,
	Manifest::{Drift, Entry, Write},
	Release::{
		Join,
		Step::{Run, Struct::Struct as Step},
		Struct::Struct as Release,
	},
};

pub fn Fn(Root:&Path, Option:&Release, Steps:&[Step]) -> Result<()> {
	Promote::Fn(Root, &Option.Version)?;

	Entry::Fn(Root)
		.iter()
		.try_for_each(|Manifest| Write::Fn(Manifest, &Option.Version))?;

	let Stale = Drift::Fn(Root, &Option.Version)?;

	Stale
		.is_empty()
		.then_some(())
		.ok_or_else(|| anyhow!("{} did not take everywhere: {}", Option.Version, Join::Fn(&Stale)))?;

	println!("{} written to every manifest.", Option.Version);

	Steps.iter().try_for_each(|Step| Run::Fn(Step, Root))
}
