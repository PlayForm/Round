//! Setting the ink.

use tauri::{AppHandle, Runtime};

use crate::{
	Menu::{
		State::Struct as State,
		Store::{Commit, Load},
	},
	Mode::Of as ModeOf,
	Per::Set as PerSet,
	Settings::Struct::Struct as Settings,
};

/// Set the ink one monitor's corners are drawn in, or every monitor's when
/// `Label` is absent.
#[tauri::command(rename = "set_mode")]
pub fn Mode<R:Runtime>(App:AppHandle<R>, Label:Option<String>, Mode:String) -> Result<State, String> {
	crate::Trace::Fn(&format!("click set_mode label={Label:?} mode={Mode}"));

	let mut Preferences:Settings = Load::Fn(&App)?;

	PerSet::Fn(&mut Preferences.Ink, Label.as_deref(), ModeOf::Fn(&Mode));

	Commit::Fn(&App, &Preferences)
}
