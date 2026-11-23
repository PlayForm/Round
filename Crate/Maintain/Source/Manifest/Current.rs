//! The version every manifest agrees on, taken from `Cargo.toml`.

use std::path::Path;

use anyhow::{Result, anyhow};

use crate::{
	Manifest::{Entry, Enum::Enum as Kind, Read},
	Version::Struct::Struct as Number,
};

/// `[workspace.package].version` is the truth: crates.io and `Publish.yml` read
/// it through `cargo metadata`, and every crate inherits it.
pub fn Fn(Root:&Path) -> Result<Number> {
	let Cargo = Entry::Fn(Root)
		.into_iter()
		.find(|Manifest| Manifest.Kind == Kind::Cargo)
		.ok_or_else(|| anyhow!("{} is not one of the manifests", Root.join("Cargo.toml").display()))?;

	Read::Fn(&Cargo)?.ok_or_else(|| anyhow!("{} has no [workspace.package].version", Cargo.Path.display()))
}
