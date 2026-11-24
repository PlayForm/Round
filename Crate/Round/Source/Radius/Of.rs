//! Reading a radius back from the store.

use serde_json::Value;

use crate::Radius::Struct::Struct;

/// Read a radius back from the store, or `None` when the value is not a number.
///
/// The conversion is the clamp, so a stored value outside the panel's range
/// comes back as the nearest one it offers rather than as an error at startup.
pub fn Fn(Value:&Value) -> Option<Struct> { Value.as_i64().map(Struct::from) }
