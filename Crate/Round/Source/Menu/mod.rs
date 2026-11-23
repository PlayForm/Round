//! The Menu window: the app's controls, in a panel that opens under the tray
//! icon and behaves like that icon's menu.
//!
//! It is Round's only control surface - the tray is an icon and a click, with
//! no native menu behind it. Everything the app can be told to do lives here,
//! including what a list of menu items could never express: a radius per
//! monitor, a value you can read while dragging it. It is a window, but it is
//! used as a menu: it opens on click, hugs its content, never appears in the
//! taskbar and never sits in the dock.
//!
//! ## How it closes
//!
//! Three layers decide it, and only one of them can dismiss the panel:
//!
//! - the corner **art** is click-through, so a click on it is a click on the
//!   desktop;
//! - the **scrim** ([`Backdrop`](crate::Backdrop)) exists only while the panel
//!   is open and takes the mouse - a `pointerdown` on it is the dismissal;
//! - the **panel** is a window one level above the scrim, so a click on the
//!   panel is delivered to the panel and never reaches the scrim underneath.
//!
//! Nothing here reads focus, activation, a timer or the pointer's position. The
//! panel used to dismiss itself on `WindowEvent::Focused(false)` behind four
//! guards - a settle window after `show`, an `NSApplication.isActive` probe, a
//! scan for another focused window of the app, and a pointer-containment test -
//! and every one of them was an inference about an event that never states its
//! cause. Each produced a wrong answer: the panel closed under a control being
//! used, then closed on a click inside the app. The scrim replaces all four
//! with an event that says what happened.

use serde::Serialize;
use tauri::{
	AppHandle,
	Manager,
	PhysicalPosition,
	Position,
	Rect,
	Runtime,
	Size,
	WebviewUrl,
	WebviewWindow,
	WebviewWindowBuilder,
};
use tauri_plugin_store::StoreExt;

use crate::{Backdrop, Mode::Mode, Overlay, Screen::Screen, Settings::Settings};

/// The window label, and the one label the overlay broadcast skips.
pub const Label:&str = "menu";

/// The panel's size in logical pixels, before the webview measures its content.
const Width:f64 = 320.0;

const Height:f64 = 340.0;

/// The gap between the tray icon and the panel, in physical pixels.
const Gap:f64 = 6.0;

/// What the panel renders.
#[derive(Clone, Debug, Serialize)]
pub struct State {
	pub size:i64,
	pub mode:&'static str,
	pub hidden:bool,
	pub dock_hidden:bool,
	pub maximum:i64,
	pub monitors:Vec<Monitor>,
}

#[derive(Clone, Debug, Serialize)]
pub struct Monitor {
	pub label:String,
	pub name:String,
	pub size:i64,
	pub overridden:bool,
}

/// Show the panel under `anchor`, or hide it when it is already open.
///
/// `anchor` is the tray icon's rect: the panel opens under the icon, on
/// whichever screen that icon lives on. The icon is outside the scrim - the
/// scrims stop at each monitor's work area - so this click is delivered to the
/// menu bar rather than swallowed, which is what keeps the tray usable while
/// the panel is open.
pub fn toggle<R:Runtime>(app:&AppHandle<R>, anchor:Rect) -> tauri::Result<()> {
	let window = match app.get_webview_window(Label) {
		Some(window) => window,
		None => build(app)?,
	};

	match window.is_visible().unwrap_or(false) {
		true => hide(app, &window),
		false => open(app, &window, anchor),
	}
}

/// Close the panel and the scrims together.
///
/// Idempotent: hiding a window that is already hidden is a no-op, so a second
/// `dismiss`, or one that arrives after the tray already closed the panel, is
/// harmless. The scrims are closed first: a scrim without a panel over it is a
/// window that eats clicks for no reason, so it must never be the one left
/// behind.
fn hide<R:Runtime>(app:&AppHandle<R>, window:&WebviewWindow<R>) -> tauri::Result<()> {
	Backdrop::close(app);

	window.hide()
}

/// Open the scrims, then the panel above them.
fn open<R:Runtime>(app:&AppHandle<R>, window:&WebviewWindow<R>, anchor:Rect) -> tauri::Result<()> {
	place(app, window, anchor);

	// The scrims go up first, so the panel is the window ordered on top of
	// them. A scrim that cannot be built is not a reason to refuse the panel -
	// the tray still closes it - so the failure is logged and the panel shows.
	if let Err(error) = Backdrop::open(app) {
		eprintln!("round: the backdrop could not be opened: {error}");
	}

	window.show()?;

	// The panel is above the scrim by construction, not by having been shown
	// later: a level on macOS, a re-insert at the top of the topmost band on
	// Windows, a raise on X11. See [`Overlay`](crate::Overlay).
	Overlay::panel(window);

	// `show` already orders the window front and makes it key; this is what
	// asks the app to activate. If macOS refuses - it refuses an activation
	// that did not come from a user gesture - the panel is still visible and
	// nothing dismisses it for that.
	window.set_focus()
}

fn build<R:Runtime>(app:&AppHandle<R>) -> tauri::Result<WebviewWindow<R>> {
	let window = WebviewWindowBuilder::new(app, Label, WebviewUrl::App("menu.html".into()))
		.title("Round")
		.inner_size(Width, Height)
		.decorations(false)
		.transparent(true)
		.resizable(false)
		.always_on_top(true)
		.skip_taskbar(true)
		.shadow(true)
		.visible(false)
		.build()?;

	Overlay::panel(&window);

	Ok(window)
}

fn place<R:Runtime>(app:&AppHandle<R>, window:&WebviewWindow<R>, anchor:Rect) {
	let scale = window.scale_factor().unwrap_or(1.0);

	let (x, y, width, height) = physical(&anchor, scale);

	let (panel_width, panel_height) = window.outer_size().map_or((Width * scale, Height * scale), |size| {
		(f64::from(size.width), f64::from(size.height))
	});

	// The screen the icon lives on, not the primary one: an icon on a second
	// display must not drag the panel back to the first.
	let (left_edge, top_edge, right_edge, bottom_edge) =
		app.monitor_from_point(x, y)
			.ok()
			.flatten()
			.map_or((0.0, 0.0, f64::MAX, f64::MAX), |monitor| {
				let position = monitor.position();

				let size = monitor.size();

				(
					f64::from(position.x),
					f64::from(position.y),
					f64::from(position.x) + f64::from(size.width),
					f64::from(position.y) + f64::from(size.height),
				)
			});

	let left = (x + width / 2.0 - panel_width / 2.0).clamp(left_edge, (right_edge - panel_width).max(left_edge));

	// Under the icon when there is room, above it otherwise - a tray at the
	// bottom of the screen (Windows) would push the panel off it.
	let below = y + height + Gap;

	let top = if below + panel_height <= bottom_edge { below } else { (y - panel_height - Gap).max(top_edge) };

	// Placement is a nicety: a panel in the wrong place is still a panel, so a
	// failure here must never abort the show.
	let _ = window.set_position(PhysicalPosition::new(left, top));
}

/// The tray rect as physical pixels, whichever unit the platform reported.
fn physical(anchor:&Rect, scale:f64) -> (f64, f64, f64, f64) {
	let (x, y) = match anchor.position {
		Position::Physical(position) => (f64::from(position.x), f64::from(position.y)),
		Position::Logical(position) => (position.x * scale, position.y * scale),
	};

	let (width, height) = match anchor.size {
		Size::Physical(size) => (f64::from(size.width), f64::from(size.height)),
		Size::Logical(size) => (size.width * scale, size.height * scale),
	};

	(x, y, width, height)
}

// --- Commands ----------------------------------------------------------------

/// Close the panel and the scrims.
///
/// This is the scrim's only command, and the only path that dismisses the panel
/// on its own: a pointer-down delivered to a window of this app that is not the
/// panel. Nothing about it reads focus, activation, timing or geometry.
#[tauri::command]
pub fn dismiss(app:AppHandle) {
	match app.get_webview_window(Label) {
		Some(window) => {
			if let Err(error) = hide(&app, &window) {
				eprintln!("round: the panel could not be closed: {error}");
			}
		}
		// A scrim outliving its panel is the one state worse than an open
		// panel, so a dismissal still closes the scrims when there is no panel
		// window left to hide.
		None => Backdrop::close(&app),
	}
}

#[tauri::command]
pub fn state(app:AppHandle) -> Result<State, String> { read(&app) }

#[tauri::command]
pub fn set_size(app:AppHandle, label:Option<String>, value:i64) -> Result<State, String> {
	let mut settings = load(&app)?;

	settings.set(label.as_deref(), value);

	commit(&app, &settings)
}

#[tauri::command]
pub fn set_mode(app:AppHandle, mode:String) -> Result<State, String> {
	let mut settings = load(&app)?;

	settings.mode = Mode::of(&mode);

	commit(&app, &settings)
}

#[tauri::command]
pub fn set_hidden(app:AppHandle, hidden:bool) -> Result<State, String> {
	let mut settings = load(&app)?;

	settings.hidden = hidden;

	commit(&app, &settings)
}

#[tauri::command]
pub fn set_dock(app:AppHandle, hidden:bool) -> Result<State, String> {
	let mut settings = load(&app)?;

	settings.dock_hidden = hidden;

	Overlay::dock(!hidden);

	commit(&app, &settings)
}

/// Drop one monitor's override, or every override and the default.
#[tauri::command]
pub fn reset(app:AppHandle, label:Option<String>) -> Result<State, String> {
	let mut settings = load(&app)?;

	settings.clear(label.as_deref());

	commit(&app, &settings)
}

#[tauri::command]
pub fn quit(app:AppHandle) { app.exit(0); }

// --- Plumbing ----------------------------------------------------------------

fn load(app:&AppHandle) -> Result<Settings, String> {
	let store = app.store(Settings::Store).map_err(|error| error.to_string())?;

	Ok(Settings::load(&store))
}

fn commit(app:&AppHandle, settings:&Settings) -> Result<State, String> {
	let store = app.store(Settings::Store).map_err(|error| error.to_string())?;

	settings.save(&store).map_err(|error| error.to_string())?;

	settings.broadcast(app);

	read(app)
}

fn read(app:&AppHandle) -> Result<State, String> {
	let store = app.store(Settings::Store).map_err(|error| error.to_string())?;

	let settings = Settings::load(&store);

	let monitors = app
		.available_monitors()
		.map_err(|error| error.to_string())?
		.iter()
		.enumerate()
		.map(|(index, monitor)| {
			let screen = Screen::of(monitor, index);

			Monitor {
				size:settings.size_for(&screen.label),
				overridden:settings.sizes.contains_key(&screen.label),
				label:screen.label,
				name:monitor.name().cloned().unwrap_or_else(|| format!("Monitor {}", index + 1)),
			}
		})
		.collect();

	Ok(State {
		size:settings.size,
		mode:settings.mode.as_str(),
		hidden:settings.hidden,
		dock_hidden:settings.dock_hidden,
		maximum:Settings::Maximum,
		monitors,
	})
}
