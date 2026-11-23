//! Setting one radius.

use crate::Settings::{Maximum, Struct::Struct};

/// Set one monitor's radius, or the default when `Label` is `None`.
///
/// Setting the default drops every per-monitor override: "apply this to all
/// monitors" is what the caller meant, and keeping stale overrides would make
/// the slider appear not to work.
pub fn Fn(State:&mut Struct, Label:Option<&str>, Value:i64) {
	let Value = Value.clamp(0, Maximum::Const);

	match Label {
		Some(Label) => {
			State.Sizes.insert(Label.to_owned(), Value);
		}
		None => {
			State.Size = Value;

			State.Sizes.clear();
		}
	}
}
