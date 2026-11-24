//! Setting one monitor's radius, or the default.

use tauri::{AppHandle, Runtime};

use crate::{
	Menu::{
		State::Struct as State,
		Store::{Commit, Load},
	},
	Per::Set as PerSet,
	Radius::Struct::Struct as Radius,
	Settings::Struct::Struct as Settings,
};

/// Set one monitor's radius, or every monitor's when `Label` is absent.
///
/// `Value` becomes a [`Radius`] here, which is where the clamp lives: nothing
/// past this point can hold a radius the panel could not have produced.
#[tauri::command(rename = "set_size")]
pub fn Size<R:Runtime>(App:AppHandle<R>, Label:Option<String>, Value:i64) -> Result<State, String> {
	crate::Trace!("click set_size label={Label:?} value={Value}");

	let mut Preferences:Settings = Load::Fn(&App)?;

	PerSet::Fn(&mut Preferences.Radius, Label.as_deref(), Radius::from(Value));

	Commit::Fn(&App, &Preferences)
}
