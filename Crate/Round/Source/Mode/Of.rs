//! Reading a mode back from the store.

use crate::Mode::Enum::Enum;

/// Read a mode back from the store.
///
/// Anything unrecognised - a hand-edited settings file, a value written by a
/// future version - falls back to the same default a fresh install gets, never
/// an error at startup.
pub fn Fn(Value:&str) -> Enum {
	match Value {
		"light" => Enum::Light,
		_ => Enum::Dark,
	}
}
