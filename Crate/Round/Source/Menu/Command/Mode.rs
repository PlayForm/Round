//! Setting the ink.

use tauri::AppHandle;

use crate::{
	Menu::{
		State::Struct as State,
		Store::{Commit, Load},
	},
	Mode::Of as ModeOf,
	Settings::Struct::Struct as Settings,
};

/// Set the ink every corner is drawn in.
#[tauri::command(rename = "set_mode")]
pub fn Mode(App:AppHandle, Mode:String) -> Result<State, String> {
	let mut Preferences:Settings = Load::Fn(&App)?;

	Preferences.Mode = ModeOf::Fn(&Mode);

	Commit::Fn(&App, &Preferences)
}
