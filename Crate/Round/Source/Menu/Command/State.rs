//! The state the panel renders.

use tauri::{AppHandle, Runtime};
use tauri_plugin_store::StoreExt;

use crate::{
	Menu::{Monitor::Struct as Monitor, State::Struct as State, Value::Of as ValueOf},
	Radius::Maximum,
	Screen::Registry,
	Settings::{Load as SettingsLoad, Store},
};

/// The whole of what the panel draws.
///
/// This is also the `state` command, so the plumbing has one reader: every
/// other command ends by calling it rather than re-deriving the same answer.
///
/// The monitors are resolved as one set, because that is the only way a label
/// can be unique: the panel values its scope-selector entries by that label, so
/// two displays of the same name have to arrive here under two of them or the
/// second entry shows the first one's settings.
#[tauri::command(rename = "state")]
pub fn State<R:Runtime>(App:AppHandle<R>) -> Result<State, String> {
	use tauri::Manager;

	let Storage = App.store(Store::Const).map_err(|Error| Error.to_string())?;

	// The set the windows were built from, not a fresh enumeration: a label is
	// unique only within a set, so resolving it twice is how the panel ends up
	// naming a monitor no window answers to.
	let Screens = App
		.try_state::<Registry::Struct>()
		.map(|Held| Held.0.clone())
		.unwrap_or_default();

	let Preferences = SettingsLoad::Fn(&Storage, &Screens);

	let Monitors = Screens
		.iter()
		.enumerate()
		.map(|(Index, Screen)| {
			let Label = Screen.Label.clone();

			Monitor {
				Radius:ValueOf::Fn(&Preferences.Radius, &Label),
				Ink:ValueOf::Fn(&Preferences.Ink, &Label),
				Hidden:ValueOf::Fn(&Preferences.Visible, &Label),
				Label,
				Name:match Screen.Name.is_empty() {
					true => format!("Monitor {}", Index + 1),
					false => Screen.Name.clone(),
				},
				Width:Screen.Pixels.0 as u32,
				Height:Screen.Pixels.1 as u32,
			}
		})
		.collect();

	Ok(State {
		Radius:Preferences.Radius.Default,
		Maximum:Maximum::Const,
		Ink:Preferences.Ink.Default,
		Hidden:Preferences.Visible.Default,
		Dock:Preferences.Dock,
		Monitors,
	})
}
