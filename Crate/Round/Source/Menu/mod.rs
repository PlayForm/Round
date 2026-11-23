//! The Menu window: the app's controls, in a panel that opens under the tray
//! icon and behaves like that icon's menu.
//!
//! The tray menu itself is native - `muda` draws it, so it cannot be HTML.
//! Everything a list of menu items cannot express (a radius per monitor, a value
//! you can read while dragging it) lives in this window instead. It is a window,
//! but it is used as a menu: it opens on click, closes when it loses focus,
//! hugs its content, never appears in the taskbar and never sits in the dock.

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

const Blur:Duration = Duration::from_millis(250);

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
/// `anchor` is the tray icon's rect; without one (the menu entry) the panel
/// centres itself.
pub fn toggle<R:Runtime>(app:&AppHandle<R>, anchor:Option<Rect>) -> tauri::Result<()> {
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

	place(app, &window, anchor)?;

	window.show()?;

	window.set_focus()
}

fn blurred_now() -> bool { Blurred.lock().ok().and_then(|at| *at).is_some_and(|at| at.elapsed() < Blur) }

fn build<R:Runtime>(app:&AppHandle<R>) -> tauri::Result<WebviewWindow<R>> {
	let window = WebviewWindowBuilder::new(app, Label, WebviewUrl::App("menu.html".into()))
		.title("Round")
		.inner_size(320.0, 340.0)
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
			if let Ok(mut at) = Blurred.lock() {
				*at = Some(Instant::now());
			}

			let _ = handle.hide();
		}
	});

	Ok(window)
}

fn place<R:Runtime>(app:&AppHandle<R>, window:&WebviewWindow<R>, anchor:Option<Rect>) -> tauri::Result<()> {
	let Some(anchor) = anchor else {
		return window.center();
	};

	let scale = window.scale_factor().unwrap_or(1.0);

	let (x, y, width, height) = physical(&anchor, scale);

	let panel = window.outer_size()?;

	let panel_width = f64::from(panel.width);

	let left = x + width / 2.0 - panel_width / 2.0;

	// Keep the panel on the screen the tray icon lives on; an icon near the right
	// edge would otherwise push half the panel off-screen.
	let limit = app
		.primary_monitor()?
		.map(|monitor| f64::from(monitor.size().width) - panel_width);

	window.set_position(PhysicalPosition::new(
		left.max(0.0).min(limit.unwrap_or(f64::MAX)),
		y + height + 6.0,
	))
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
