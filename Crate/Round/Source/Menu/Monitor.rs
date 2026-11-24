//! One monitor as the panel names it.

use serde::Serialize;

use crate::{
	Menu::Value::Struct::Struct as Value,
	Mode::Enum::Enum as Ink,
	Radius::Struct::Struct as Radius,
	Visibility::Enum::Enum as Visible,
};

/// A monitor as the panel lists it, with every setting already resolved.
///
/// The three settings are [`Value`]s rather than plain values because each one
/// shows whether its value is this monitor's own choice, and only the Rust side
/// holds the override map that answers that.
///
/// `Width`/`Height` are the physical pixels the display reports, and they are
/// here for the one job no setting does: telling two monitors apart. A name is
/// not an identity - two displays of the same model arrive with the same one, and
/// a truncated one is ambiguous even when the full names differ - so the panel
/// prints the resolution beside the name wherever it has to name a single
/// monitor. They are read from the device rather than from [`Screen`], which
/// carries the logical bounds the overlay window is built with: the number worth
/// printing is the one printed on the box.
///
/// [`Screen`]: crate::Screen::Struct::Struct
#[derive(Clone, Debug, Serialize)]
pub struct Struct {
	#[serde(rename = "label")]
	pub Label:String,
	#[serde(rename = "name")]
	pub Name:String,
	#[serde(rename = "width")]
	pub Width:u32,
	#[serde(rename = "height")]
	pub Height:u32,
	#[serde(rename = "radius")]
	pub Radius:Value<Radius>,
	#[serde(rename = "ink")]
	pub Ink:Value<Ink>,
	#[serde(rename = "hidden")]
	pub Hidden:Value<Visible>,
}
