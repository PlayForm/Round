//! Every manifest whose version differs from the one just written.

use std::path::{Path, PathBuf};

use anyhow::Result;

use crate::{
	Manifest::{Entry, Read},
	Version::Struct::Struct as Number,
};

pub fn Fn(Root:&Path, Version:&Number) -> Result<Vec<PathBuf>> {
	let Held:Vec<(PathBuf, Option<Number>)> = Entry::Fn(Root)
		.into_iter()
		.map(|Manifest| Read::Fn(&Manifest).map(|Held| (Manifest.Path, Held)))
		.collect::<Result<_>>()?;

	Ok(Held
		.into_iter()
		.filter_map(|(Path, Held)| Held.filter(|Held| Held != Version).map(|_| Path))
		.collect())
}
