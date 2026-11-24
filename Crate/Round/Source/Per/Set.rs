//! Writing a value into one monitor's setting, or into the default.

use crate::Per::Struct::Struct;

/// Set `Label`'s value, or the default when `Label` is `None`.
///
/// Setting the default drops every override: "apply this to all monitors" is
/// what the caller meant, and keeping stale overrides would make the control
/// appear not to work.
pub fn Fn<T>(State:&mut Struct<T>, Label:Option<&str>, Value:T) {
	match Label {
		Some(Label) => {
			State.Monitor.insert(Label.to_owned(), Value);
		}
		None => {
			State.Default = Value;

			State.Monitor.clear();
		}
	}
}
