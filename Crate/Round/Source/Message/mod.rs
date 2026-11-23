//! What the tray broadcasts to the corner windows.

use serde::Serialize;

use crate::Mode::Mode;

/// A state change pushed to every window.
///
/// The shape is the frontend contract: `listen("mode", ({ payload }) =>
/// payload.message.Mode)`. Externally tagged, lowercase mode, integer size -
/// changing either arm changes the webview.
#[derive(Clone, Copy, Debug, Serialize)]
pub enum Message {
	Mode(Mode),
	Size(i64),
}

/// The envelope Tauri hands to a `listen` callback.
#[derive(Clone, Copy, Debug, Serialize)]
pub struct Payload {
	pub message:Message,
}
