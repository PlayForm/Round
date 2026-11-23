//! Writing one version into every manifest.

use std::path::Path;

use anyhow::Result;

use crate::{
	CLI::Report,
	Manifest::{Entry, Write},
	Version::Struct::Struct as Number,
};

/// The write is followed by the read: a version command reports the table it
/// leaves behind, so a manifest that refused the number is visible immediately.
pub fn Fn(Root:&Path, Version:&Number) -> Result<()> {
	Entry::Fn(Root).iter().try_for_each(|Manifest| Write::Fn(Manifest, Version))?;

	Report::Fn(Root)
}
