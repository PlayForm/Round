//! The state the panel renders.

use tauri::AppHandle;
use tauri_plugin_store::StoreExt;

use crate::{
	Menu::{Monitor::Struct as Monitor, State::Struct as State, Value::Of as ValueOf},
	Radius::Maximum,
	Screen::All as ScreenAll,
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
pub fn State(App:AppHandle) -> Result<State, String> {
	let Storage = App.store(Store::Const).map_err(|Error| Error.to_string())?;

	let Devices = App.available_monitors().map_err(|Error| Error.to_string())?;

	let Screens = ScreenAll::Fn(&Devices);

	let Preferences = SettingsLoad::Fn(&Storage, &Screens);

	let Monitors = Devices
		.iter()
		.zip(Screens)
		.enumerate()
		.map(|(Index, (Device, Screen))| {
			let Label = Screen.Label;

			// The physical pixels, for the panel to print beside the name: the
			// overlay's own bounds in `Screen` are logical and would name a
			// display by a number its owner does not recognise.
			let Extent = Device.size();

			Monitor {
				Radius:ValueOf::Fn(&Preferences.Radius, &Label),
				Ink:ValueOf::Fn(&Preferences.Ink, &Label),
				Hidden:ValueOf::Fn(&Preferences.Visible, &Label),
				Label,
				Name:Device.name().cloned().unwrap_or_else(|| format!("Monitor {}", Index + 1)),
				Width:Extent.width,
				Height:Extent.height,
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
