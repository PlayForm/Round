//! Showing or hiding the dock icon.

use tauri::{AppHandle, Runtime};

use crate::{
	Menu::{
		State::Struct as State,
		Store::{Commit, Load},
	},
	Overlay::{Layer::Trait::Trait as _, Platform::Struct as Platform},
	Settings::Struct::Struct as Settings,
	Visibility::Of as VisibilityOf,
};

/// Show or hide the dock icon, and persist the choice.
///
/// The activation policy is process-wide and is applied here rather than read
/// back from the store, so the icon changes on the click instead of on the next
/// launch.
#[tauri::command(rename = "set_dock")]
pub fn Dock<R:Runtime>(App:AppHandle<R>, Hidden:bool) -> Result<State, String> {
	crate::Trace!("click set_dock hidden={Hidden}");

	let mut Preferences:Settings = Load::Fn(&App)?;

	Preferences.Dock = VisibilityOf::Fn(Hidden);

	Platform::Dock(Preferences.Dock);

	Commit::Fn(&App, &Preferences)
}
