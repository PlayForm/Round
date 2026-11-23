//! Reading the persisted state back.

use tauri::Runtime;
use tauri_plugin_store::Store;

use crate::{Mode::Of as ModeOf, Settings::Struct::Struct, Visibility::Of as VisibilityOf};

/// Read the settings back, falling back per field to [`Struct::default`].
///
/// A hand-edited or truncated store can therefore cost one setting, never the
/// launch. Every key is spelled out rather than derived: they are the shape of
/// `settings.json`, which an older build of Round wrote and a newer one reads.
pub fn Fn<R:Runtime>(Storage:&Store<R>) -> Struct {
	let Default = Struct::default();

	Struct {
		Size:Storage.get("size").and_then(|Value| Value.as_i64()).unwrap_or(Default.Size),
		Sizes:Storage
			.get("sizes")
			.and_then(|Value| Value.as_object().cloned())
			.map(|Object| {
				Object
					.into_iter()
					.filter_map(|(Label, Value)| Value.as_i64().map(|Size| (Label, Size)))
					.collect()
			})
			.unwrap_or_default(),
		Mode:Storage
			.get("mode")
			.and_then(|Value| Value.as_str().map(ModeOf::Fn))
			.unwrap_or(Default.Mode),
		Hidden:Storage
			.get("hidden")
			.and_then(|Value| Value.as_bool())
			.map(VisibilityOf::Fn)
			.unwrap_or(Default.Hidden),
		Dock:Storage
			.get("dock_hidden")
			.and_then(|Value| Value.as_bool())
			.map(VisibilityOf::Fn)
			.unwrap_or(Default.Dock),
	}
}
