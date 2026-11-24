//! Rebuilding the status item so it is drawn unpressed.

use tauri::{AppHandle, Runtime};

use crate::Tray::Label;

/// Replace the status item with a fresh one, drawn in its normal state.
///
/// The public tray surface cannot clear the pressed state - no method exposes the
/// status-bar button the highlight lives on - but `set_visible` is the whole
/// status item: `false` removes it, `true` builds a new one from the attributes
/// the tray was created with, and a button that did not exist when the click was
/// handled cannot be carrying that click's highlight.
///
/// Both calls are attempted even when the first one fails. A removal that fails
/// leaves the icon where it was, which is the state we started in; a removal that
/// succeeded and a re-create that did not would leave no icon at all, so the
/// second call must never be skipped.
///
/// The re-created item keeps its place: `create()` re-adds the status item with
/// the same attributes and the same id, so it is drawn with the template flag it
/// was built with and its click still routes to the handler registered against
/// that id. It is also placed back in the same slot rather than at the end of the
/// menu bar - measured by removing and re-creating one status item six times in a
/// probe app and reading the status window's origin each time: the same `x` every
/// cycle, and the new button reporting unpressed every cycle.
pub fn Fn<R:Runtime>(App:&AppHandle<R>) {
	let Some(Tray) = App.tray_by_id(Label::Const) else {
		eprintln!(
			"round: no tray icon is registered as `{}`, so a pressed icon cannot be cleared",
			Label::Const
		);

		return;
	};

	let _ = Tray
		.set_visible(false)
		.inspect_err(|Error| eprintln!("round: the tray icon could not be removed: {Error}"));

	let _ = Tray
		.set_visible(true)
		.inspect_err(|Error| eprintln!("round: the tray icon could not be restored: {Error}"));
}
