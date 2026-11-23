//! Showing one scrim over every monitor.

use tauri::{AppHandle, Manager, Runtime};

use crate::Backdrop::{Label, Owns, Show};

/// Show one scrim over every monitor's work area, building the ones that do not
/// exist yet.
///
/// A monitor whose scrim cannot be built is logged and skipped rather than
/// aborting the open: a panel with a scrim on two of three displays is still a
/// panel, and the tray icon still closes it.
pub fn Fn<R:Runtime>(App:&AppHandle<R>) -> tauri::Result<()> {
	let Monitors = App.available_monitors()?;

	Monitors.iter().enumerate().for_each(|(Index, Monitor)| {
		let _ = Show::Fn(App, Monitor, Index)
			.inspect_err(|Error| eprintln!("round: the backdrop over monitor {Index} could not be shown: {Error}"));
	});

	// A display that went away takes its scrim with it. A stale one would sit
	// over a screen that no longer exists and eat clicks with no panel above it,
	// which is the one state worse than a panel that will not close.
	let Live:Vec<String> = (0..Monitors.len()).map(Label::Fn).collect();

	App.webview_windows()
		.into_values()
		.filter(|Window| Owns::Fn(Window.label()) && !Live.iter().any(|Label| Label == Window.label()))
		.for_each(|Window| {
			let _ = Window.hide();
		});

	Ok(())
}
