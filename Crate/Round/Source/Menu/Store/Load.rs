//! Reading the store for a command.

use tauri::{AppHandle, Runtime};
use tauri_plugin_store::StoreExt;

use crate::{
	Screen::Registry,
	Settings::{Load as SettingsLoad, Store, Struct::Struct as Settings},
};

/// The settings as they are on disk right now, keyed by the labels this launch
/// gives the monitors.
///
/// Read per command rather than held: the panel is the only writer, and a copy
/// that outlived a write would answer the next command with a stale radius. The
/// labels come from the set the windows were built from, not from a fresh
/// enumeration - a label is unique only within a set, so resolving it twice is
/// how a write lands on a key no window reads.
pub fn Fn<R:Runtime>(App:&AppHandle<R>) -> Result<Settings, String> {
	use tauri::Manager;

	let Storage = App.store(Store::Const).map_err(|Error| Error.to_string())?;

	let Screens = App
		.try_state::<Registry::Struct>()
		.map(|Held| Held.0.clone())
		.unwrap_or_default();

	Ok(SettingsLoad::Fn(&Storage, &Screens))
}
