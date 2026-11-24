//! Folding a store written before the radius had per-monitor overrides.

use std::collections::BTreeMap;

use tauri::Runtime;
use tauri_plugin_store::Store;

use crate::{Per::Struct::Struct as Per, Radius::Struct::Struct as Radius};

/// The radius an older store held, or `None` when it held none.
///
/// Before this shape the radius was `size` for every monitor and `sizes` for the
/// tuned ones - exactly one [`Per`], spelled differently. A store holding neither
/// key is not one this migration has anything to say about, so it answers `None`
/// and the caller takes the fresh-install default.
pub fn Fn<R:Runtime>(Storage:&Store<R>) -> Option<Per<Radius>> {
	let Default = Storage.get("size").and_then(|Value| Value.as_i64()).map(Radius::from);

	let Monitor:BTreeMap<String, Radius> = Storage
		.get("sizes")
		.and_then(|Value| Value.as_object().cloned())
		.map(|Object| {
			Object
				.into_iter()
				.filter_map(|(Label, Value)| Value.as_i64().map(|Value| (Label, Radius::from(Value))))
				.collect()
		})
		.unwrap_or_default();

	let Held = Default.is_some() || !Monitor.is_empty();

	Held.then_some(Per { Default:Default.unwrap_or_default(), Monitor })
}
