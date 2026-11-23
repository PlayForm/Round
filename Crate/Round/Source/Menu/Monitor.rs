//! One row of the panel's monitor list.

use serde::Serialize;

/// A monitor as the panel lists it.
///
/// `Override` is a fact about the store, not a two-state decision: it says
/// whether this monitor has an entry of its own, and there is no third answer a
/// two-variant enum could not express but no second reading it could.
#[derive(Clone, Debug, Serialize)]
pub struct Struct {
	#[serde(rename = "label")]
	pub Label:String,
	#[serde(rename = "name")]
	pub Name:String,
	#[serde(rename = "size")]
	pub Size:i64,
	#[serde(rename = "overridden")]
	pub Override:bool,
}
