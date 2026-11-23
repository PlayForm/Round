//! What the tray broadcasts to the corner windows.

use serde::Serialize;

use crate::Mode::Enum::Enum as Mode;

/// A state change pushed to every window.
///
/// The shape is the frontend contract: `listen("mode", ({ payload }) =>
/// payload.message.Mode)`. Externally tagged, lowercase mode, integer size -
/// changing either arm changes the webview.
#[derive(Clone, Copy, Debug, Serialize)]
pub enum Enum {
	Mode(Mode),
	Size(i64),
}
