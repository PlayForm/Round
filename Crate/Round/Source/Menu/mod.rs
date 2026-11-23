//! The Menu window: the app's controls, in a panel that opens under the tray
//! icon and behaves like that icon's menu.
//!
//! It is Round's only control surface - the tray is an icon and a click, with
//! no native menu behind it. Everything the app can be told to do lives here,
//! including what a list of menu items could never express: a radius per
//! monitor, a value you can read while dragging it. It is a window, but it is
//! used as a menu: it opens on click, closes when it loses focus, hugs its
//! content, never appears in the taskbar and never sits in the dock.

use std::{
	sync::Mutex,
	time::{Duration, Instant},
};

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
	WindowEvent,
};
use tauri_plugin_store::StoreExt;

use crate::{Mode::Mode, Overlay, Screen::Screen, Settings::Settings};

/// The window label, and the one label the overlay broadcast skips.
pub const Label:&str = "menu";

/// When the panel last hid because it lost focus.
///
/// Clicking the tray icon while the panel is open blurs it first, so a naive
/// toggle would hide it and immediately show it again. A click that arrives
/// inside this window after a blur is the dismissal, not a new opening.
static Blurred:Mutex<Option<Instant>> = Mutex::new(None);

/// When the panel was last shown.
///
/// Ordering a window front keys it and then drops it again in the same tick:
/// macOS activates the app only after the window is already key, and refuses
/// that activation when the show did not come from a user gesture. A blur that
/// arrives this soon after `show` is the show settling, not a dismissal.
static Shown:Mutex<Option<Instant>> = Mutex::new(None);

const Blur:Duration = Duration::from_millis(250);

/// How long after `show` a focus loss still counts as the show settling.
const Settle:Duration = Duration::from_millis(500);

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
/// whichever screen that icon lives on.
pub fn toggle<R:Runtime>(app:&AppHandle<R>, anchor:Rect) -> tauri::Result<()> {
	let window = match app.get_webview_window(Label) {
		Some(window) => window,
		None => build(app)?,
	};

	if window.is_visible().unwrap_or(false) {
		return window.hide();
	}

	// The click that dismissed it also blurred it; treat that as "closed".
	if blurred_now() {
		return Ok(());
	}

	place(app, &window, anchor);

	// Recorded before `show`, because the focus change can arrive inside it.
	if let Ok(mut at) = Shown.lock() {
		*at = Some(Instant::now());
	}

	window.show()?;

	// `show` already orders the window front and makes it key; this is what
	// asks the app to activate. If macOS refuses, the panel is still visible
	// and the settle window below keeps the refusal from hiding it.
	window.set_focus()
}

fn blurred_now() -> bool { Blurred.lock().ok().and_then(|at| *at).is_some_and(|at| at.elapsed() < Blur) }

fn settling() -> bool { Shown.lock().ok().and_then(|at| *at).is_some_and(|at| at.elapsed() < Settle) }

/// Whether the app still holds focus when the window resigns the keyboard.
///
/// On macOS, ordering a window front keys it before the app is active, and the
/// system can deactivate that activation attempt a moment later - the window
/// then resigns the keyboard with the app already inactive. A real click on
/// another app does the opposite: the window resigns the keyboard while this
/// app is still active, and the app deactivates after. So only the latter is a
/// dismissal.
#[cfg(target_os = "macos")]
fn dismissed() -> bool {
	use objc2::{class, msg_send, runtime::AnyObject};

	unsafe {
		let app:*mut AnyObject = msg_send![class!(NSApplication), sharedApplication];

		msg_send![app, isActive]
	}
}

#[cfg(not(target_os = "macos"))]
fn dismissed() -> bool { true }

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

	// A popover dismisses when it loses focus, exactly like the menu it replaces.
	let handle = window.clone();

	window.on_window_event(move |event| {
		if let WindowEvent::Focused(false) = event {
			// Hiding a window that is already hidden emits another blur, a
			// window that was just ordered front is still settling, and an
			// activation the system took back is not the user leaving. None of
			// those is a dismissal, and recording one would swallow the next
			// tray click.
			if !handle.is_visible().unwrap_or(false) || settling() || !dismissed() {
				return;
			}

			if let Ok(mut at) = Blurred.lock() {
				*at = Some(Instant::now());
			}

			let _ = handle.hide();
		}
	});

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
