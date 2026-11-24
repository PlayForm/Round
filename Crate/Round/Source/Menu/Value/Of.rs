//! Resolving one setting for one monitor.

use crate::{
	Menu::{Origin::Enum::Enum as Origin, Value::Struct::Struct},
	Per::{Entry, Struct::Struct as Per},
};

/// `Label`'s value and where it came from.
///
/// This is the only place the panel's two halves are filled in, so a row can
/// never show a value and name the wrong origin for it.
pub fn Fn<T:Copy>(Value:&Per<T>, Label:&str) -> Struct<T> {
	match Entry::Fn(Value, Label) {
		Some(Value) => Struct { Value, Origin:Origin::Override },
		None => Struct { Value:Value.Default, Origin:Origin::Inherit },
	}
}
