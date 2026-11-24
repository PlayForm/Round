//! The state the panel renders.

use tauri::AppHandle;
use tauri_plugin_store::StoreExt;

use crate::{
	Menu::{Monitor::Struct as Monitor, State::Struct as State, Value::Of as ValueOf},
	Radius::Maximum,
	Screen::Of as ScreenOf,
	Settings::{Load as SettingsLoad, Store},
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
				Radius:ValueOf::Fn(&Preferences.Radius, &Label),
				Ink:ValueOf::Fn(&Preferences.Ink, &Label),
				Hidden:ValueOf::Fn(&Preferences.Visible, &Label),
				Label,
				Name:Device.name().cloned().unwrap_or_else(|| format!("Monitor {}", Index + 1)),
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
