//! Which field a bump moves.

use std::str::FromStr;

use anyhow::{Result, anyhow};

/// Which field a bump moves.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Enum {
	Major,
	Minor,
	Patch,
}

impl FromStr for Enum {
	type Err = anyhow::Error;

	fn from_str(Text:&str) -> Result<Self> {
		match Text {
			"major" => Ok(Self::Major),
			"minor" => Ok(Self::Minor),
			"patch" => Ok(Self::Patch),
			Other => Err(anyhow!("`{Other}` is not major, minor or patch")),
		}
	}
}
