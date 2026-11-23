//! One file that carries the version.

use std::path::PathBuf;

use crate::Manifest::Enum::Enum as Kind;

/// One file that carries the version, and how it stores it.
pub struct Struct {
	pub Kind:Kind,
	pub Path:PathBuf,
}
