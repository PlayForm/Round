//! Dropping an override.

use crate::Settings::{Default, Struct::Struct};

/// Drop one monitor's override, or every override and the default.
pub fn Fn(State:&mut Struct, Label:Option<&str>) {
	match Label {
		Some(Label) => {
			State.Sizes.remove(Label);
		}
		None => {
			State.Size = Default::Const;

			State.Sizes.clear();
		}
	}
}
