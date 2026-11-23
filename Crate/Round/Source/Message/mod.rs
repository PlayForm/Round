//! What the tray broadcasts to the corner windows.

mod Payload;

use serde::Serialize;

pub use Payload::Payload;

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
