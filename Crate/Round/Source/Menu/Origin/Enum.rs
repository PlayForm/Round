//! Where a monitor's value came from.

use serde::Serialize;

/// Whether a monitor carries its own value or follows the default.
///
/// The panel marks an overridden row differently from an inherited one, and a
/// value alone cannot say which it is: a monitor tuned to exactly the default is
/// still overridden, so a `bool` here would be this decision left unwritten.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
pub enum Enum {
	/// No entry of its own: the value shown is the default.
	#[serde(rename = "inherit")]
	Inherit,
	/// An entry of its own, whatever the value happens to be.
	#[serde(rename = "override")]
	Override,
}
