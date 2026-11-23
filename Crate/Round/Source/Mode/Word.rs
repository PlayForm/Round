//! The word a mode travels as.

use crate::Mode::Enum::Enum;

/// The lowercase word the store and the webview both carry.
///
/// `data-mode` on the `.Window` element and the `mode` store key read this
/// exact spelling, so it is a contract rather than a label.
pub fn Fn(Value:Enum) -> &'static str {
	match Value {
		Enum::Dark => "dark",
		Enum::Light => "light",
	}
}
