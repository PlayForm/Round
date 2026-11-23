//! A `major.minor.patch` version, and the arithmetic a release needs.

use std::{
	fmt::{Display, Formatter, Result as FormatResult},
	str::FromStr,
};

use crate::Error::{Error, Result};

/// Which field a bump moves.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Part {
	Major,
	Minor,
	Patch,
}

impl FromStr for Part {
	type Err = Error;

	fn from_str(text:&str) -> Result<Self> {
		match text {
			"major" => Ok(Self::Major),
			"minor" => Ok(Self::Minor),
			"patch" => Ok(Self::Patch),
			other => Err(Error::Usage(format!("`{other}` is not major, minor or patch"))),
		}
	}
}

/// The version, as every manifest stores it.
///
/// `pre` is carried but never produced by a bump: a release moves to a clean
/// `major.minor.patch`, and a pre-release is written explicitly with `set`.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Version {
	pub major:u64,
	pub minor:u64,
	pub patch:u64,
	pub pre:Option<String>,
}

impl Version {
	pub fn bump(self, part:Part) -> Self {
		match part {
			Part::Major => Self { major:self.major + 1, minor:0, patch:0, pre:None },
			Part::Minor => Self { major:self.major, minor:self.minor + 1, patch:0, pre:None },
			Part::Patch => Self { major:self.major, minor:self.minor, patch:self.patch + 1, pre:None },
		}
	}

	/// The tag this version is released under.
	pub fn tag(&self) -> String { format!("Round/v{self}") }

	/// A version is greater than another when it sorts after it, which is what
	/// the release guard needs: crates.io and tags are append-only.
	pub fn is_after(&self, other:&Self) -> bool { self > other }
}

impl Display for Version {
	fn fmt(&self, formatter:&mut Formatter<'_>) -> FormatResult {
		match &self.pre {
			Some(pre) => write!(formatter, "{}.{}.{}-{pre}", self.major, self.minor, self.patch),
			None => write!(formatter, "{}.{}.{}", self.major, self.minor, self.patch),
		}
	}
}

impl FromStr for Version {
	type Err = Error;

	fn from_str(text:&str) -> Result<Self> {
		let (core, pre) = text
			.split_once('-')
			.map(|(core, pre)| (core, Some(pre.to_owned())))
			.unwrap_or((text, None));

		let fields:Vec<&str> = core.split('.').collect();

		let [major, minor, patch] = fields.as_slice() else {
			return Err(Error::Usage(format!("`{text}` is not a semver version")));
		};

		let number = |field:&str| -> Result<u64> {
			field
				.parse::<u64>()
				.map_err(|_| Error::Usage(format!("`{text}` has a non-numeric field `{field}`")))
		};

		Ok(Self { major:number(major)?, minor:number(minor)?, patch:number(patch)?, pre })
	}
}
