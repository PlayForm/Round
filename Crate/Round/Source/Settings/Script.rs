//! The script an overlay window runs before the frontend does.

use serde_json::json;

use crate::{Per::Of as PerOf, Settings::Struct::Struct};

/// The `window.settings` initialisation script one overlay window runs before
/// the frontend does, so the first paint already has the right size and ink
/// instead of flashing the defaults.
///
/// Both values are resolved for this window's label, so a monitor with an
/// override paints as itself before the first event arrives.
pub fn Fn(State:&Struct, Label:&str) -> String {
	format!(
		"window.settings = {};",
		json!({ "size": PerOf::Fn(&State.Radius, Label), "mode": PerOf::Fn(&State.Ink, Label) })
	)
}
