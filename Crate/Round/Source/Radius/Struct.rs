//! A corner radius that is always one the panel offers.

use serde::{Serialize, Serializer};

use crate::Radius::Maximum;

/// A corner radius, in logical pixels.
///
/// The only way to make one is [`From<i64>`], which clamps to `0..=Maximum`. So
/// a hand-edited store, a stale window message or a slider that grew a step
/// cannot put a radius on screen that the panel could not have produced: the
/// clamp is not a check a caller might forget, it is the constructor.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Struct(i64);

/// The value a fresh install starts at, so `Per::Clear` needs no second table
/// of defaults.
impl Default for Struct {
	fn default() -> Self { Self(crate::Radius::Default::Const) }
}

impl From<i64> for Struct {
	fn from(Value:i64) -> Self { Self(Value.clamp(0, Maximum::Const)) }
}

/// The store and the webview both carry a radius as a JSON integer, which is the
/// one thing about this type the two of them see.
impl Serialize for Struct {
	fn serialize<S:Serializer>(&self, Serializer:S) -> Result<S::Ok, S::Error> { Serializer.serialize_i64(self.0) }
}
