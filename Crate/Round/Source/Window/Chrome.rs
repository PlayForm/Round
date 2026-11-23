//! The window flags every one of Round's windows shares.

use tauri::{Manager, Runtime, WebviewWindowBuilder};

/// A builder carrying the chrome all three layers want.
///
/// Round has no decorated window: the art, the scrim and the panel are all
/// borderless, transparent, fixed-size, always-on-top and built hidden, and the
/// only thing that differs is the title. Spelled once here so a new layer
/// cannot be born with a title bar and a resize handle, and so the three
/// windows are visibly the same window shape in the source as they are on
/// screen.
///
/// `skip_taskbar` is inert off Windows and is applied on every platform rather
/// than behind a `cfg`, because the flag states what the window is - a tool
/// surface, not a document - and a platform that ignores it is not a platform
/// that disagrees.
pub fn Fn<'A, R:Runtime, M:Manager<R>>(
	Builder:WebviewWindowBuilder<'A, R, M>,
	Title:&str,
) -> WebviewWindowBuilder<'A, R, M> {
	Builder
		.title(Title)
		.decorations(false)
		.transparent(true)
		.resizable(false)
		.always_on_top(true)
		.skip_taskbar(true)
		.visible(false)
}
