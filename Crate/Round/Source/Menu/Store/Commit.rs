//! Writing the store and answering the panel.

use tauri::{AppHandle, Runtime};
use tauri_plugin_store::StoreExt;

use crate::{
	Menu::{Command::State as CommandState, State::Struct as State},
	Settings::{Broadcast, Save as SettingsSave, Store, Struct::Struct as Settings},
};

/// Persist `Preferences`, push them to every overlay, then answer with the
/// state the panel should render.
///
/// One order, in one place: every command ends here, so no command can save
/// without broadcasting or broadcast without having saved.
pub fn Fn<R:Runtime>(App:&AppHandle<R>, Preferences:&Settings) -> Result<State, String> {
	let Storage = App.store(Store::Const).map_err(|Error| Error.to_string())?;

	SettingsSave::Fn(Preferences, &Storage).map_err(|Error| Error.to_string())?;

	Broadcast::Fn(Preferences, App);

	CommandState::State(App.clone())
}
