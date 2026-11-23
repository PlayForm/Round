//! Whether a window is wanted on screen.

use serde::{Serialize, Serializer};

/// A window's wanted visibility, as a value rather than a `bool`.
///
/// Every `bool` the overlay used to carry meant one of these two states, and a
/// `bool` at a call site reads as a condition instead of a decision. `Shown` is
/// the default because a fresh install shows its corners.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Enum {
	/// Wanted on screen.
	#[default]
	Shown,
	/// Wanted off screen.
	Hidden,
}

/// The store and the webview both carry this as a JSON boolean, so the wire
/// shape is unchanged by the two-variant enum.
impl Serialize for Enum {
	fn serialize<S:Serializer>(&self, Serializer:S) -> Result<S::Ok, S::Error> {
		Serializer.serialize_bool(crate::Visibility::Bool::Fn(*self))
	}
}
