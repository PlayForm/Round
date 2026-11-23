//! A `major.minor.patch` version, as every manifest stores it.

use std::{
	fmt::{Display, Formatter, Result as FormatResult},
	str::FromStr,
};

use anyhow::{Result, anyhow};

/// The version, as every manifest stores it.
///
/// `Pre` is carried but never produced by a bump: a release moves to a clean
/// `major.minor.patch`, and a pre-release is written explicitly with `set`.
///
/// The derived order is the release guard: crates.io and tags are append-only,
/// so a version that does not sort after the current one can only fail later,
/// after the tag exists.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Struct {
	pub Major:u64,
	pub Minor:u64,
	pub Patch:u64,
	pub Pre:Option<String>,
}

impl Display for Struct {
	fn fmt(&self, Formatter:&mut Formatter<'_>) -> FormatResult {
		match &self.Pre {
			Some(Pre) => write!(Formatter, "{}.{}.{}-{Pre}", self.Major, self.Minor, self.Patch),
			None => write!(Formatter, "{}.{}.{}", self.Major, self.Minor, self.Patch),
		}
	}
}

/// The error is a message for the caller that typed the version, so it is an
/// `anyhow` error and never a variant anybody matches.
impl FromStr for Struct {
	type Err = anyhow::Error;

	fn from_str(Text:&str) -> Result<Self> {
		let (Core, Pre) = Text
			.split_once('-')
			.map(|(Core, Pre)| (Core, Some(Pre.to_owned())))
			.unwrap_or((Text, None));

		let Fields:Vec<&str> = Core.split('.').collect();

		let [Major, Minor, Patch] = Fields.as_slice() else {
			return Err(anyhow!("`{Text}` is not a semver version"));
		};

		let Number = |Field:&str| -> Result<u64> {
			Field
				.parse::<u64>()
				.map_err(|_| anyhow!("`{Text}` has a non-numeric field `{Field}`"))
		};

		Ok(Self { Major:Number(Major)?, Minor:Number(Minor)?, Patch:Number(Patch)?, Pre })
	}
}
