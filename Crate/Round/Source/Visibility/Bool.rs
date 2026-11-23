//! Converting the value into the boolean the store and the webview carry.

use crate::Visibility::Enum::Enum;

/// The boolean `hidden` / `dock_hidden` is written as, and the one the panel
/// reads back. `true` means hidden, which is the spelling both consumers use.
pub fn Fn(Value:Enum) -> bool {
	match Value {
		Enum::Shown => false,
		Enum::Hidden => true,
	}
}
