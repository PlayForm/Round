//! Reading the persisted state back.

use tauri::Runtime;
use tauri_plugin_store::Store;

use crate::{
	Mode::Of as ModeOf,
	Per::Read as PerRead,
	Radius::Of as RadiusOf,
	Settings::{Migrate, Struct::Struct},
	Visibility::Of as VisibilityOf,
};

/// Read the settings back, falling back per setting to [`Struct::default`].
///
/// A hand-edited or truncated store can therefore cost one setting, never the
/// launch. The per-monitor keys are read by [`PerRead`], which owns the "default
/// plus overrides" shape; what is spelled out here is the store's own key names,
/// because they are the shape of `settings.json` that an older build of Round
/// wrote and a newer one reads. A key the current shape does not hold is
/// answered by [`Migrate`], which folds the store an older build wrote.
pub fn Fn<R:Runtime>(Storage:&Store<R>) -> Struct {
	Struct {
		Radius:PerRead::Fn(Storage.get("radius").as_ref(), RadiusOf::Fn)
			.or_else(|| Migrate::Radius::Fn(Storage))
			.unwrap_or_default(),
		Ink:PerRead::Fn(Storage.get("ink").as_ref(), |Value| Value.as_str().map(ModeOf::Fn))
			.or_else(|| Migrate::Ink::Fn(Storage))
			.unwrap_or_default(),
		// The old key is the new key: `hidden` held a bare boolean, which
		// `PerRead` already reads as a default with no overrides.
		Visible:PerRead::Fn(Storage.get("hidden").as_ref(), |Value| Value.as_bool().map(VisibilityOf::Fn))
			.unwrap_or_default(),
		Dock:Storage
			.get("dock_hidden")
			.and_then(|Value| Value.as_bool())
			.map(VisibilityOf::Fn)
			.unwrap_or_default(),
	}
}
