//! A setting with no overrides yet.

use std::collections::BTreeMap;

use crate::Per::Struct::Struct;

/// One setting whose default is `Default` and whose monitors all follow it.
///
/// This is the only way to make a [`Struct`] from a bare value, so "no
/// overrides" is a state the type can be handed rather than a map a caller has
/// to remember to leave empty.
pub fn Fn<T>(Default:T) -> Struct<T> { Struct { Default, Monitor:BTreeMap::new() } }
