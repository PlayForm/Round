//! The ink a corner is drawn in.

use serde::{Serialize, Serializer};

/// Two variants, because a corner is either black or white.
///
/// `Dark` draws black corners (the default, for light desktops); `Light` draws
/// white ones. The webview reads the same word as `data-mode`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Mode {
	#[default]
	Dark,
	Light,
}

impl Mode {
	pub const fn as_str(self) -> &'static str {
		match self {
			Self::Dark => "dark",
			Self::Light => "light",
		}
	}

	/// Read a mode back from the store. Anything unrecognised - a hand-edited
	/// settings file, a value written by a future version - falls back to the
	/// same default a fresh install gets, never an error at startup.
	pub fn of(value:&str) -> Self {
		match value {
			"light" => Self::Light,
			_ => Self::Dark,
		}
	}
}

impl Serialize for Mode {
	fn serialize<S:Serializer>(&self, serializer:S) -> Result<S::Ok, S::Error> {
		serializer.serialize_str(self.as_str())
	}
}
