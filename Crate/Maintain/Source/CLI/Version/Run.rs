//! The three ways to write a version, and the report that reads it.

use std::path::Path;

use anyhow::Result;

use crate::{
	CLI::{Report, Version::Enum::Enum as Action, Write},
	Manifest::Current,
	Version::Bump,
};

pub fn Fn(Root:&Path, Action:Option<Action>) -> Result<()> {
	match Action {
		Some(Action::Set { Version }) => Write::Fn(Root, &Version),
		Some(Action::Bump { Part }) => Write::Fn(Root, &Bump::Fn(Current::Fn(Root)?, Part)),
		Some(Action::Sync) => Write::Fn(Root, &Current::Fn(Root)?),
		None => Report::Fn(Root),
	}
}
