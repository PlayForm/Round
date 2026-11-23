//! Dropping an override.

use tauri::AppHandle;

use crate::{
	Menu::{
		State::Struct as State,
		Store::{Commit, Load},
	},
	Settings::{Clear, Struct::Struct as Settings},
};

/// Drop one monitor's override, or every override and the default.
#[tauri::command(rename = "reset")]
pub fn Reset(App:AppHandle, Label:Option<String>) -> Result<State, String> {
	let mut Preferences:Settings = Load::Fn(&App)?;

	Clear::Fn(&mut Preferences, Label.as_deref());

	Commit::Fn(&App, &Preferences)
}
