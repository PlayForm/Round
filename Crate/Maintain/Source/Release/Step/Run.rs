//! Running one step.

use std::path::Path;

use anyhow::Result;

use crate::{
	Process::Run,
	Release::Step::Struct::Struct as Step,
};

pub fn Fn(Step:&Step, Root:&Path) -> Result<()> {
	let Args:Vec<&str> = Step.Args.iter().map(String::as_str).collect();

	Run::Fn(Root, Step.Program, &Args)
}
