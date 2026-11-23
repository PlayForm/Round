//! Printing the plan instead of running it.

use anyhow::Result;

use crate::Release::{
	Step::{Render, Struct::Struct as Step},
	Struct::Struct as Release,
};

pub fn Fn(Option:&Release, Steps:&[Step]) -> Result<()> {
	println!(
		"would promote the changelog to {}, write {} to every manifest, then run:",
		Option.Version, Option.Version
	);

	Steps.iter().for_each(|Step| println!("{}", Render::Fn(Step)));

	Ok(())
}
