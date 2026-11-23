//! Everything Round persists between launches.

use std::collections::BTreeMap;

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
/// `size` is the default every monitor takes; `sizes` holds the monitors that
/// were tuned on their own, keyed by their window label. A monitor with no entry
/// follows the default - which is what makes "Increase" a global gesture and a
/// slider in the controls window a per-monitor one.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Settings {
	pub size:i64,
	pub sizes:BTreeMap<String, i64>,
	pub mode:Mode,
	pub hidden:bool,
	/// macOS only in the UI; kept on every platform so the store shape and this
	/// struct never diverge by target.
	pub dock_hidden:bool,
}

impl Default for Settings {
	fn default() -> Self {
		Self { size:Self::Size, sizes:BTreeMap::new(), mode:Mode::Dark, hidden:false, dock_hidden:false }
	}
}

impl Settings {
	/// Corner radius, in logical pixels, on a fresh install.
	pub const Size:i64 = 23;

	/// How far one Increase/Decrease click moves the radius.
	pub const Step:i64 = 6;

	/// The largest radius the controls window offers.
	pub const Maximum:i64 = 120;

	/// The store file `tauri-plugin-store` writes under the app config dir.
	pub const Store:&'static str = "settings.json";

	/// The radius one monitor is drawn with.
	pub fn size_for(&self, label:&str) -> i64 { self.sizes.get(label).copied().unwrap_or(self.size) }

	/// Set one monitor's radius, or the default when `label` is `None`.
	///
	/// Setting the default drops every per-monitor override: "apply this to all
	/// monitors" is what the caller meant, and keeping stale overrides would make
	/// the slider appear not to work.
	pub fn set(&mut self, label:Option<&str>, value:i64) {
		let value = value.clamp(0, Self::Maximum);

		match label {
			Some(label) => {
				self.sizes.insert(label.to_owned(), value);
			},
			None => {
				self.size = value;

				self.sizes.clear();
			},
		}
	}

	/// Drop one monitor's override, or every override and the default.
	pub fn clear(&mut self, label:Option<&str>) {
		match label {
			Some(label) => {
				self.sizes.remove(label);
			},
			None => {
				self.size = Self::Size;

				self.sizes.clear();
			},
		}
	}

	/// Move the default and every override together, so a global step keeps the
	/// per-monitor relationships a user set up.
	fn shift(&mut self, delta:i64) {
		self.size = (self.size + delta).clamp(0, Self::Maximum);

		self.sizes.iter_mut().for_each(|(_, size)| *size = (*size + delta).clamp(0, Self::Maximum));
	}

	/// Apply one tray command and say what the caller does next.
	///
	/// The whole menu is one match over a domain enum: no string comparison
	/// survives past [`Action::of`], and the follow-up work is returned as an
	/// [`Effect`] instead of being re-derived at the call site.
	pub fn next(mut self, action:Action) -> (Self, Effect) {
		match action {
			Action::Increase => self.shift(Self::Step),
			Action::Decrease => self.shift(-Self::Step),
			Action::Reset => self.clear(None),
			Action::Dark => self.mode = Mode::Dark,
			Action::Light => self.mode = Mode::Light,
			Action::Show => self.hidden = false,
			Action::Hide => self.hidden = true,
			Action::Dock => {
				self.dock_hidden = !self.dock_hidden;

				let hidden = self.dock_hidden;

				return (self, Effect::Dock(hidden));
			},
			Action::Controls => return (self, Effect::Controls),
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
			sizes:store
				.get("sizes")
				.and_then(|value| value.as_object().cloned())
				.map(|object| {
					object
						.into_iter()
						.filter_map(|(label, value)| value.as_i64().map(|size| (label, size)))
						.collect()
				})
				.unwrap_or_default(),
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

		store.set("sizes", json!(self.sizes));

		store.set("mode", json!(self.mode));

		store.set("hidden", json!(self.hidden));

		store.set("dock_hidden", json!(self.dock_hidden));

		store.save()
	}

	/// The `window.settings` initialisation script one overlay window runs
	/// before the frontend does, so the first paint already has the right size
	/// and ink instead of flashing the defaults.
	pub fn script(&self, label:&str) -> String {
		format!("window.settings = {};", json!({ "size": self.size_for(label), "mode": self.mode }))
	}

	/// Show or hide one overlay window to match the persisted visibility.
	pub fn visible<R:Runtime>(&self, window:&WebviewWindow<R>) -> tauri::Result<()> {
		match self.hidden {
			true => window.hide(),
			false => window.show(),
		}
	}

	/// Push the current state to every overlay window.
	///
	/// The controls window is skipped: it is an ordinary window, and hiding or
	/// resizing it with the overlays would take away the panel the user is using.
	pub fn broadcast<R:Runtime>(&self, app:&AppHandle<R>) {
		app.webview_windows()
			.into_values()
			.filter(|window| window.label() != crate::Controls::Label)
			.for_each(|window| {
				let label = window.label().to_owned();

				let _ = window.emit("size", Payload { message:Message::Size(self.size_for(&label)) });

				let _ = window.emit("mode", Payload { message:Message::Mode(self.mode) });

				let _ = self.visible(&window);
			});
	}
}
