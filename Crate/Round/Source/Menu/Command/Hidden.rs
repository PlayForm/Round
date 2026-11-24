//! Showing or hiding the corners.

use tauri::{AppHandle, Runtime};

use crate::{
	Menu::{
		State::Struct as State,
		Store::{Commit, Load},
	},
	Per::Set as PerSet,
	Settings::Struct::Struct as Settings,
	Visibility::Of as VisibilityOf,
};

/// Show or hide one monitor's corners, or every monitor's when `Label` is
/// absent.
#[tauri::command(rename = "set_hidden")]
pub fn Hidden<R:Runtime>(App:AppHandle<R>, Label:Option<String>, Hidden:bool) -> Result<State, String> {
	crate::Trace!("click set_hidden label={Label:?} hidden={Hidden}");

	let mut Preferences:Settings = Load::Fn(&App)?;

	PerSet::Fn(&mut Preferences.Visible, Label.as_deref(), VisibilityOf::Fn(Hidden));

	Commit::Fn(&App, &Preferences)
}
