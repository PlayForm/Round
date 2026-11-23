//! What the panel renders.

use serde::Serialize;

use crate::{Menu::Monitor::Struct as Monitor, Mode::Enum::Enum as Mode, Visibility::Enum::Enum as Visibility};

/// The whole of what the panel draws, returned by every command.
///
/// Every key is pinned with a per-field `rename` rather than a container
/// attribute: the field names are PascalCase by house style and the wire keys
/// are the lowercase words `Menu.tsx` reads, so deriving one from the other is
/// the one thing this struct cannot do.
#[derive(Clone, Debug, Serialize)]
pub struct Struct {
	#[serde(rename = "size")]
	pub Size:i64,
	#[serde(rename = "mode")]
	pub Mode:Mode,
	#[serde(rename = "hidden")]
	pub Hidden:Visibility,
	#[serde(rename = "dock_hidden")]
	pub Dock:Visibility,
	#[serde(rename = "maximum")]
	pub Maximum:i64,
	#[serde(rename = "monitors")]
	pub Monitors:Vec<Monitor>,
}
