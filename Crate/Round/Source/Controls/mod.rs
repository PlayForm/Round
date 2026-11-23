//! The controls window: an HTML panel, one slider per monitor.
//!
//! The tray menu is native (`muda` draws it, so it cannot be HTML); everything
//! the menu cannot express - a radius per monitor, a live preview, a value you
//! can read - lives here instead of being crammed into more menu entries.

use serde::Serialize;
use tauri::{AppHandle, Manager, Runtime, WebviewUrl, WebviewWindowBuilder};
use tauri_plugin_store::StoreExt;

use crate::Mode::Mode;
use crate::Screen::Screen;
use crate::Settings::Settings;
use crate::Overlay;

/// The window label, and the one label the overlay broadcast skips.
pub const Label:&str = "controls";

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

/// Open the panel, or bring the existing one forward.
pub fn open<R:Runtime>(app:&AppHandle<R>) -> tauri::Result<()> {
	app.get_webview_window(Label).map_or_else(
		|| {
			WebviewWindowBuilder::new(app, Label, WebviewUrl::App("controls.html".into()))
				.title("Round - Controls")
				.inner_size(420.0, 560.0)
				.min_inner_size(380.0, 420.0)
				.resizable(true)
				.center()
				.build()
				.map(|_| ())
		},
		|window| {
			let _ = window.show();

			window.set_focus()
		},
	)
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
