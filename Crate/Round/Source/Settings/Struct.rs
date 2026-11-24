//! The persisted state of the overlay.

use crate::{
	Mode::Enum::Enum as Ink,
	Per::Struct::Struct as Per,
	Radius::Struct::Struct as Radius,
	Visibility::Enum::Enum as Visible,
};

/// Everything Round persists between launches.
///
/// The three per-monitor settings are [`Per`] rather than a default and a map
/// written out three times: each holds the value every monitor takes and the
/// monitors tuned on their own, so a monitor with no entry follows the default
/// and one gesture changes all of them at once.
///
/// `Dock` is the one setting with no per-monitor dimension: the dock icon
/// belongs to the application, not to a monitor, so there is nothing to override
/// per screen.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Struct {
	pub Radius:Per<Radius>,
	pub Ink:Per<Ink>,
	pub Visible:Per<Visible>,
	/// macOS only in the UI; kept on every platform so the store shape and this
	/// struct never diverge by target.
	pub Dock:Visible,
}
