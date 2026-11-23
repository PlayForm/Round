//! The tag a version is released under.

use crate::Version::Struct::Struct as Number;

/// `Build.yml` keys off this prefix, so it is the contract between the CLI and
/// the release workflow rather than a label.
pub fn Fn(Version:&Number) -> String { format!("Round/v{Version}") }
