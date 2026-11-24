//! Dropping an override.

use crate::Per::Struct::Struct;

/// Drop `Label`'s override, or every override and the default.
///
/// Clearing the default is a reset rather than a deletion: a setting always has
/// a default, so what it means is "the value a fresh install would have", which
/// is why the bound is [`Default`] and not `Option`.
pub fn Fn<T:Default>(State:&mut Struct<T>, Label:Option<&str>) {
	match Label {
		Some(Label) => {
			State.Monitor.remove(Label);
		}
		None => {
			State.Default = T::default();

			State.Monitor.clear();
		}
	}
}
