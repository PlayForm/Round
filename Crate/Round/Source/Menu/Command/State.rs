//! The state the panel renders.

use tauri::AppHandle;
use tauri_plugin_store::StoreExt;

use crate::{
	Menu::{Monitor::Struct as Monitor, State::Struct as State},
	Screen::Of as ScreenOf,
	Settings::{Load as SettingsLoad, Maximum, Radius, Store},
};

/// The whole of what the panel draws.
///
/// This is also the `state` command, so the plumbing has one reader: every
/// other command ends by calling it rather than re-deriving the same answer.
#[tauri::command(rename = "state")]
pub fn State(App:AppHandle) -> Result<State, String> {
	let Storage = App.store(Store::Const).map_err(|Error| Error.to_string())?;

	let Preferences = SettingsLoad::Fn(&Storage);

	let Monitors = App
		.available_monitors()
		.map_err(|Error| Error.to_string())?
		.iter()
		.enumerate()
		.map(|(Index, Device)| {
			let Screen = ScreenOf::Fn(Device, Index);

			let Label = Screen.Label;

			Monitor {
				Size:Radius::Fn(&Preferences, &Label),
				Override:Preferences.Sizes.contains_key(&Label),
				Label,
				Name:Device.name().cloned().unwrap_or_else(|| format!("Monitor {}", Index + 1)),
			}
		})
		.collect();

	Ok(State {
		Size:Preferences.Size,
		Mode:Preferences.Mode,
		Hidden:Preferences.Hidden,
		Dock:Preferences.Dock,
		Maximum:Maximum::Const,
		Monitors,
	})
}
