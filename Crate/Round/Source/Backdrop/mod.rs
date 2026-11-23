//! The scrim: the window that catches the click which closes the panel.
//!
//! A popover has to decide when it was dismissed, and the OS will not say. A
//! blur states that the keyboard moved, not that the user left; a pointer
//! position states where the mouse was, not what it did. So Round puts a window
//! of its own over every monitor while the panel is open, and lets the click
//! arrive: a `pointerdown` delivered to this document is a fact - the pointer
//! went down outside the panel - and the panel is hidden from that event alone.
//!
//! It covers each monitor's **work area**, not its whole rect: the menu bar
//! (and, on macOS, the Dock strip; on Windows, the taskbar) stays outside the
//! scrim, so a click on the tray icon still reaches the icon and toggles the
//! panel closed. A click in that strip goes to the OS chrome instead, which
//! activates whatever it landed on and leaves the panel open - the one shape of
//! click the scrim cannot see, and the price of never swallowing the tray.
//!
//! The scrims are built on the first open and reused after it, hidden while the
//! panel is closed. Each carries its own monitor's work area in physical pixels,
//! so a desk that mixes a Retina display with a 1x one, or puts a display left
//! of the main one at a negative origin, needs no arithmetic: every window is
//! told exactly where its own screen's work area is.

use tauri::{
	AppHandle,
	Manager,
	Monitor,
	PhysicalPosition,
	PhysicalSize,
	Runtime,
	WebviewUrl,
	WebviewWindow,
	WebviewWindowBuilder,
};

use crate::Overlay;

/// The prefix every scrim window label carries.
const Prefix:&str = "backdrop";

/// The label of the scrim over the monitor at `index`.
///
/// Indexed rather than named after the monitor: a window is built once and
/// re-placed on every open, so nothing depends on which display it covered the
/// time it was created, and a display that is plugged back in under another
/// name still lands on a window that already exists.
fn label(index:usize) -> String { format!("{Prefix}{index}") }

/// Whether `label` names a scrim window.
///
/// The overlay broadcast asks this before treating a window as art: the scrim
/// is hidden and shown by the panel, and a broadcast that "restored" it would
/// leave a click-eating window over the desktop with no panel on it.
pub fn owns(label:&str) -> bool { label.starts_with(Prefix) }

/// Show one scrim over every monitor's work area, building the ones that do not
/// exist yet.
///
/// A monitor whose scrim cannot be built is logged and skipped rather than
/// aborting the open: a panel with a scrim on two of three displays is still a
/// panel, and the tray icon still closes it.
pub fn open<R:Runtime>(app:&AppHandle<R>) -> tauri::Result<()> {
	let monitors = app.available_monitors()?;

	monitors.iter().enumerate().for_each(|(index, monitor)| {
		if let Err(error) = show(app, monitor, index) {
			eprintln!("round: the backdrop over monitor {index} could not be shown: {error}");
		}
	});

	// A display that went away takes its scrim with it. A stale one would sit
	// over a screen that no longer exists and eat clicks with no panel above
	// it, which is the one state worse than a panel that will not close.
	let live:Vec<String> = (0..monitors.len()).map(label).collect();

	app.webview_windows()
		.into_values()
		.filter(|window| owns(window.label()) && !live.iter().any(|label| label == window.label()))
		.for_each(|window| {
			let _ = window.hide();
		});

	Ok(())
}

/// Hide every scrim.
///
/// Idempotent: a window that is already hidden is left hidden, so closing
/// twice costs nothing and cannot fail - the scrim's second click, or a
/// `dismiss` that arrives after the tray already closed the panel.
pub fn close<R:Runtime>(app:&AppHandle<R>) {
	app.webview_windows()
		.into_values()
		.filter(|window| owns(window.label()))
		.for_each(|window| {
			let _ = window.hide();
		});
}

/// Place, size and show one monitor's scrim.
fn show<R:Runtime>(app:&AppHandle<R>, monitor:&Monitor, index:usize) -> tauri::Result<()> {
	let label = label(index);

	let window = match app.get_webview_window(&label) {
		Some(window) => window,
		None => build(app, &label)?,
	};

	let area = monitor.work_area();

	// Physical pixels, exactly as the platform reports them: the builder only
	// takes logical units, and rounding a fractional scale factor is how a
	// screen edge ends up a pixel outside the scrim with a click that never
	// dismisses.
	window.set_position(PhysicalPosition::new(area.position.x, area.position.y))?;

	window.set_size(PhysicalSize::new(area.size.width, area.size.height))?;

	// Above the art, below the panel, and in the mouse's way. Applied on every
	// open, so a window that was reused carries the ladder it needs today.
	Overlay::scrim(&window);

	Overlay::front(&window)
}

fn build<R:Runtime>(app:&AppHandle<R>, label:&str) -> tauri::Result<WebviewWindow<R>> {
	WebviewWindowBuilder::new(app, label, WebviewUrl::App("backdrop.html".into()))
		.title("")
		.decorations(false)
		.transparent(true)
		.resizable(false)
		.always_on_top(true)
		.skip_taskbar(true)
		.shadow(false)
		// Not focusable, so a click on the scrim cannot take the keyboard from
		// the panel or from the app the user was typing in; `accept_first_mouse`
		// then makes that same click count even when Round is inactive, so the
		// dismissal is the click the user made rather than the one after it.
		.focusable(false)
		.accept_first_mouse(true)
		.focused(false)
		.disable_drag_drop_handler()
		.visible(false)
		.build()
}
