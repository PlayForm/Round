//! Showing or hiding the corners.

use tauri::AppHandle;

use crate::{
	Menu::{
		State::Struct as State,
		Store::{Commit, Load},
	},
	Settings::Struct::Struct as Settings,
	Visibility::Of as VisibilityOf,
};

/// Show or hide every corner overlay.
#[tauri::command(rename = "set_hidden")]
pub fn Hidden(App:AppHandle, Hidden:bool) -> Result<State, String> {
	let mut Preferences:Settings = Load::Fn(&App)?;

	Preferences.Hidden = VisibilityOf::Fn(Hidden);

	Commit::Fn(&App, &Preferences)
}
