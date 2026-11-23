//! The script an overlay window runs before the frontend does.

use serde_json::json;

use crate::Settings::{Radius, Struct::Struct};

/// The `window.settings` initialisation script one overlay window runs before
/// the frontend does, so the first paint already has the right size and ink
/// instead of flashing the defaults.
pub fn Fn(State:&Struct, Label:&str) -> String {
	format!(
		"window.settings = {};",
		json!({ "size": Radius::Fn(State, Label), "mode": State.Mode })
	)
}
