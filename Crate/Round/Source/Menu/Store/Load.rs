//! Reading the store for a command.

use tauri::AppHandle;
use tauri_plugin_store::StoreExt;

use crate::Settings::{Load as SettingsLoad, Store, Struct::Struct as Settings};

/// The settings as they are on disk right now.
///
/// Read per command rather than held: the panel is the only writer, and a copy
/// that outlived a write would answer the next command with a stale radius.
pub fn Fn(App:&AppHandle) -> Result<Settings, String> {
	let Storage = App.store(Store::Const).map_err(|Error| Error.to_string())?;

	Ok(SettingsLoad::Fn(&Storage))
}
