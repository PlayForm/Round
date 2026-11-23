//! The envelope a window's `listen` callback receives.

use serde::Serialize;

use crate::Message::Enum::Enum;

/// The envelope Tauri hands to a `listen` callback.
///
/// The key is pinned rather than derived: a field named `Message` would
/// serialise as `Message`, and the frontend reads `payload.message`.
#[derive(Clone, Copy, Debug, Serialize)]
pub struct Struct {
	#[serde(rename = "message")]
	pub Message:Enum,
}
