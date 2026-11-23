//! Setting one monitor's radius, or the default.

use tauri::AppHandle;

use crate::{
	Menu::{
		State::Struct as State,
		Store::{Commit, Load},
	},
	Settings::{Set, Struct::Struct as Settings},
};

/// Set one monitor's radius, or every monitor's when `Label` is absent.
#[tauri::command(rename = "set_size")]
pub fn Size(App:AppHandle, Label:Option<String>, Value:i64) -> Result<State, String> {
	let mut Preferences:Settings = Load::Fn(&App)?;

	Set::Fn(&mut Preferences, Label.as_deref(), Value);

	Commit::Fn(&App, &Preferences)
}
