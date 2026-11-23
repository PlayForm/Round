//! The persisted state of the overlay.

use std::collections::BTreeMap;

use crate::{Mode::Enum::Enum as Mode, Visibility::Enum::Enum as Visibility};

/// Everything Round persists between launches.
///
/// `Size` is the default every monitor takes; `Sizes` holds the monitors that
/// were tuned on their own, keyed by their window label. A monitor with no entry
/// follows the default - which is what makes the all-monitors slider one gesture
/// and a per-monitor slider a local one.
///
/// `Hidden` and `Dock` are [`Visibility`] rather than `bool`: each one is a
/// two-state decision, and a `bool` field is that decision left unwritten.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Struct {
	pub Size:i64,
	pub Sizes:BTreeMap<String, i64>,
	pub Mode:Mode,
	pub Hidden:Visibility,
	/// macOS only in the UI; kept on every platform so the store shape and this
	/// struct never diverge by target.
	pub Dock:Visibility,
}

impl Default for Struct {
	fn default() -> Self {
		Self {
			Size:crate::Settings::Default::Const,
			Sizes:BTreeMap::new(),
			Mode:Mode::default(),
			Hidden:Visibility::default(),
			Dock:Visibility::default(),
		}
	}
}
