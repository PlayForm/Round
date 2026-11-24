//! Folding a store written when the ink was global.

use tauri::Runtime;
use tauri_plugin_store::Store;

use crate::{
	Mode::{Enum::Enum as Ink, Of as ModeOf},
	Per::{Build, Struct::Struct as Per},
};

/// The ink an older store held, or `None` when it held none.
///
/// The ink used to be one `mode` for every monitor. A store written since holds
/// `ink` instead, which is the key the caller reads first; this reads only the
/// old one, so a monitor tuned on its own is not something an older file could
/// have recorded.
pub fn Fn<R:Runtime>(Storage:&Store<R>) -> Option<Per<Ink>> {
	Storage
		.get("mode")
		.and_then(|Value| Value.as_str().map(ModeOf::Fn))
		.map(Build::Fn)
}
