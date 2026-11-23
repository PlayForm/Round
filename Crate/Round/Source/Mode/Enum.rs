//! The ink a corner is drawn in.

use serde::{Serialize, Serializer};

/// Two variants, because a corner is either black or white.
///
/// `Dark` draws black corners (the default, for light desktops); `Light` draws
/// white ones. The webview reads the same word as `data-mode`, and
/// [`Word`](crate::Mode::Word) is the one place that spelling is decided.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Enum {
	/// Black corners - the default, for a light desktop.
	#[default]
	Dark,
	/// White corners, for a dark desktop.
	Light,
}

impl Serialize for Enum {
	fn serialize<S:Serializer>(&self, Serializer:S) -> Result<S::Ok, S::Error> { Serializer.serialize_str(crate::Mode::Word::Fn(*self)) }
}
