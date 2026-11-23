//! A manifest's path as a human reads it.

use std::path::Path;

use crate::Manifest::Struct::Struct as Manifest;

/// The path under the repository root, so the table and the `git add` list name
/// the same file the reader opened.
pub fn Fn(Root:&Path, Manifest:&Manifest) -> String {
	Manifest.Path.strip_prefix(Root).unwrap_or(&Manifest.Path).display().to_string()
}
