//! One row of the panel's monitor list.

use serde::Serialize;

use crate::{
	Menu::Value::Struct::Struct as Value,
	Mode::Enum::Enum as Ink,
	Radius::Struct::Struct as Radius,
	Visibility::Enum::Enum as Visible,
};

/// A monitor as the panel lists it.
///
/// The three settings are [`Value`]s rather than plain values because each row
/// shows whether its value is this monitor's own choice, and only the Rust side
/// holds the override map that answers that.
#[derive(Clone, Debug, Serialize)]
pub struct Struct {
	#[serde(rename = "label")]
	pub Label:String,
	#[serde(rename = "name")]
	pub Name:String,
	#[serde(rename = "radius")]
	pub Radius:Value<Radius>,
	#[serde(rename = "ink")]
	pub Ink:Value<Ink>,
	#[serde(rename = "hidden")]
	pub Hidden:Value<Visible>,
}
