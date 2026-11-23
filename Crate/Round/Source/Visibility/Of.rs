//! Converting between the stored boolean and the value.

use crate::Visibility::Enum::Enum;

/// The value a stored or received boolean names.
///
/// `true` means hidden: the store keys (`hidden`, `dock_hidden`) are written
/// as "is it hidden", so the conversion carries that spelling rather than
/// re-deciding it at every reader.
pub fn Fn(Hidden:bool) -> Enum {
	match Hidden {
		true => Enum::Hidden,
		false => Enum::Shown,
	}
}
