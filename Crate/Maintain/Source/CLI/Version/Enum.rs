//! What `Maintain version` does when it is not just reporting.

use clap::Subcommand;

use crate::Version::{Enum::Enum as Part, Struct::Struct as Number};

#[derive(Subcommand)]
pub enum Enum {
	/// Write this version to every manifest.
	Set { Version:Number },
	/// Move one field and write the result everywhere.
	Bump { Part:Part },
	/// Re-propagate `Cargo.toml`'s version after editing it by hand.
	Sync,
}
