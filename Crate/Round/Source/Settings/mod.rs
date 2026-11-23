//! Everything Round persists between launches.

use serde_json::json;
use tauri::{AppHandle, Emitter, Manager, Runtime, WebviewWindow};
use tauri_plugin_store::Store;

use crate::{
	Action::Action,
	Effect::Effect,
	Message::{Message, Payload},
	Mode::Mode,
};

/// The persisted state of the overlay.
///
/// Four scalars, one of them a [`Mode`] rather than a `String`: the store's
/// on-disk shape is this struct's serialisation, so an unknown mode cannot
/// reach the webview through a typo.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Settings {
	pub size:i64,
	pub mode:Mode,
	pub hidden:bool,
	/// macOS only in the UI; kept on every platform so the store shape and this
	/// struct never diverge by target.
	pub dock_hidden:bool,
}

impl Default for Settings {
	fn default() -> Self { Self { size:Self::Size, mode:Mode::Dark, hidden:false, dock_hidden:false } }
}

impl Settings {
	/// Corner radius, in logical pixels, on a fresh install.
	pub const Size:i64 = 23;

	/// How far one Increase/Decrease click moves the radius.
	pub const Step:i64 = 6;

	/// The store file `tauri-plugin-store` writes under the app config dir.
	pub const Store:&'static str = "settings.json";

	/// Apply one tray command and say what the caller does next.
	///
	/// The whole menu is one match over a domain enum: no string comparison
	/// survives past [`Action::of`], and the follow-up work is returned as an
	/// [`Effect`] instead of being re-derived at the call site.
	pub fn next(mut self, action:Action) -> (Self, Effect) {
		match action {
			Action::Increase => self.size = self.size.saturating_add(Self::Step),
			Action::Decrease => self.size = self.size.saturating_sub(Self::Step).max(0),
			Action::Reset => self.size = Self::Size,
			Action::Dark => self.mode = Mode::Dark,
			Action::Light => self.mode = Mode::Light,
			Action::Show => self.hidden = false,
			Action::Hide => self.hidden = true,
			Action::Dock => {
				self.dock_hidden = !self.dock_hidden;

				return (self, Effect::Dock(self.dock_hidden));
			}
			Action::Exit => return (self, Effect::Exit),
		}

		(self, Effect::Persist)
	}

	/// Read the settings back, falling back per field to [`Settings::default`].
	/// A hand-edited or truncated store can therefore cost one setting, never
	/// the launch.
	pub fn load<R:Runtime>(store:&Store<R>) -> Self {
		let default = Self::default();

		Self {
			size:store.get("size").and_then(|value| value.as_i64()).unwrap_or(default.size),
			mode:store
				.get("mode")
				.and_then(|value| value.as_str().map(Mode::of))
				.unwrap_or(default.mode),
			hidden:store.get("hidden").and_then(|value| value.as_bool()).unwrap_or(default.hidden),
			dock_hidden:store
				.get("dock_hidden")
				.and_then(|value| value.as_bool())
				.unwrap_or(default.dock_hidden),
		}
	}

	pub fn save<R:Runtime>(&self, store:&Store<R>) -> tauri_plugin_store::Result<()> {
		store.set("size", json!(self.size));

		store.set("mode", json!(self.mode));

		store.set("hidden", json!(self.hidden));

		store.set("dock_hidden", json!(self.dock_hidden));

		store.save()
	}

	/// The `window.settings` initialisation script each overlay window runs
	/// before the frontend does, so the first paint already has the right size
	/// and ink instead of flashing the defaults.
	pub fn script(&self) -> String { format!("window.settings = {};", json!({ "size": self.size, "mode": self.mode })) }

	/// Show or hide one overlay window to match the persisted visibility.
	pub fn visible<R:Runtime>(&self, window:&WebviewWindow<R>) -> tauri::Result<()> {
		match self.hidden {
			true => window.hide(),
			false => window.show(),
		}
	}

	/// Push the current state to every overlay window.
	pub fn broadcast<R:Runtime>(&self, app:&AppHandle<R>) {
		app.webview_windows().into_values().for_each(|window| {
			let _ = window.emit("size", Payload { message:Message::Size(self.size) });

			let _ = window.emit("mode", Payload { message:Message::Mode(self.mode) });

			let _ = self.visible(&window);
		});
	}
}
