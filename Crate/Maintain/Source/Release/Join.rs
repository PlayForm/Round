//! The stale manifests, as one line.

use std::path::PathBuf;

pub fn Fn(Paths:&[PathBuf]) -> String {
	Paths.iter().map(|Path| Path.display().to_string()).collect::<Vec<_>>().join(", ")
}
