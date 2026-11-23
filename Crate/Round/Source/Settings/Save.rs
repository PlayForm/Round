//! Writing the state back.

use serde_json::json;
use tauri::Runtime;
use tauri_plugin_store::Store;

use crate::Settings::Struct::Struct;

/// Write every setting, then flush the store to disk.
pub fn Fn<R:Runtime>(State:&Struct, Storage:&Store<R>) -> tauri_plugin_store::Result<()> {
	Storage.set("size", json!(State.Size));

	Storage.set("sizes", json!(State.Sizes));

	Storage.set("mode", json!(State.Mode));

	Storage.set("hidden", json!(State.Hidden));

	Storage.set("dock_hidden", json!(State.Dock));

	Storage.save()
}
