use serde::Serialize;

use super::Message;

/// The envelope Tauri hands to a `listen` callback.
#[derive(Clone, Copy, Debug, Serialize)]
pub struct Payload {
	pub message: Message,
}
