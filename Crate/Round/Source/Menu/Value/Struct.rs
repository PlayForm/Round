//! One setting as one monitor sees it.

use serde::Serialize;

use crate::Menu::Origin::Enum::Enum as Origin;

/// The value a monitor is drawn with, and where it came from.
///
/// The panel cannot work either half out for itself - it does not hold the
/// override map - so both travel: `Value` is what the row's control shows, and
/// `Origin` is whether that value is this monitor's own.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
pub struct Struct<T> {
	#[serde(rename = "value")]
	pub Value:T,
	#[serde(rename = "origin")]
	pub Origin:Origin,
}
