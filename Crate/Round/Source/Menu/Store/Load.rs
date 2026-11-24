//! Reading the store for a command.

use tauri::AppHandle;
use tauri_plugin_store::StoreExt;

use crate::{
	Screen::All as ScreenAll,
	Settings::{Load as SettingsLoad, Store, Struct::Struct as Settings},
};

/// The settings as they are on disk right now, keyed by the labels this launch
/// gives the monitors.
///
/// Read per command rather than held: the panel is the only writer, and a copy
/// that outlived a write would answer the next command with a stale radius. The
/// monitors are read with it because the store's keys are labels, and the label
/// a monitor carries is a fact about the set - a display plugged in since the
/// last command changes it.
pub fn Fn(App:&AppHandle) -> Result<Settings, String> {
	let Storage = App.store(Store::Const).map_err(|Error| Error.to_string())?;

	let Monitors = App.available_monitors().map_err(|Error| Error.to_string())?;

	Ok(SettingsLoad::Fn(&Storage, &ScreenAll::Fn(&Monitors)))
}
