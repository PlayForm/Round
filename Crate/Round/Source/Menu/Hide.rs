//! Close the panel and the scrims together.

use tauri::{AppHandle, Runtime, WebviewWindow};

use crate::{Backdrop::Close as BackdropClose, Tray::Session::Release};

/// Close the panel and the scrims together.
///
/// Idempotent: hiding a window that is already hidden is a no-op, so a second
/// `dismiss`, or one that arrives after the tray already closed the panel, is
/// harmless. The scrims are closed first: a scrim without a panel over it is a
/// window that eats clicks for no reason, so it must never be the one left
/// behind.
///
/// This is also where the tray's menu session ends, which is the one signal the
/// platform does not send: a right click leaves the icon drawn pressed with
/// nothing left to clear it, so the close releases the session and unpresses the
/// icon. Only the close does it - a command that runs while the panel stays open
/// must not make the icon blink.
pub fn Fn<R:Runtime>(App:&AppHandle<R>, Window:&WebviewWindow<R>) -> tauri::Result<()> {
	BackdropClose::Fn(App);

	let Closed = Window.hide();

	Release::Fn(App);

	Closed
}
