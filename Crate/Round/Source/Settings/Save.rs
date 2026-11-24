//! Writing the state back.

use serde_json::json;
use tauri::Runtime;
use tauri_plugin_store::Store;

use crate::Settings::Struct::Struct;

/// Write every setting, then flush the store to disk.
///
/// The three per-monitor settings are written as the shape
/// [`Per`](crate::Per::Struct::Struct) serialises to (`{"default": ...,
/// "monitor": {...}}`), so the reader and the writer agree by construction instead
/// of by two lists of keys kept in step by hand.
pub fn Fn<R:Runtime>(State:&Struct, Storage:&Store<R>) -> tauri_plugin_store::Result<()> {
	Storage.set("radius", json!(State.Radius));

	Storage.set("ink", json!(State.Ink));

	Storage.set("hidden", json!(State.Visible));

	Storage.set("dock_hidden", json!(State.Dock));

	Storage.save()
}
