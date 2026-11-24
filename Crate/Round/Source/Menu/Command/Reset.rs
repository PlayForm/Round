//! Dropping an override.

use tauri::{AppHandle, Runtime};

use crate::{
	Menu::{
		Setting::Clear as SettingClear,
		State::Struct as State,
		Store::{Commit, Load},
	},
	Settings::Struct::Struct as Settings,
};

/// Drop one monitor's override, or every override and the default.
///
/// `Setting` names the panel section the button belongs to (`radius`, `ink`,
/// `hidden`); absent means all three, which is what a reset with no section
/// named has always meant.
#[tauri::command(rename = "reset")]
pub fn Reset<R:Runtime>(App:AppHandle<R>, Setting:Option<String>, Label:Option<String>) -> Result<State, String> {
	crate::Trace!("click reset setting={Setting:?} label={Label:?}");

	let mut Preferences:Settings = Load::Fn(&App)?;

	SettingClear::Fn(&mut Preferences, Setting.as_deref(), Label.as_deref())?;

	Commit::Fn(&App, &Preferences)
}
