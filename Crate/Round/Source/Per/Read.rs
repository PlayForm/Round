//! Reading one per-monitor setting back from the store.

use serde_json::Value;

use crate::Per::{Build, Struct::Struct};

/// Read one setting, or `None` when the store holds nothing usable.
///
/// `Parse` is the only thing that differs between the three settings - how a
/// stored value becomes the type - so the "default plus overrides" shape is read
/// here once. A bare value is taken as the default with no overrides: that is
/// what an older store wrote for a setting that was global, and what a
/// hand-edited file would naturally say. Anything unreadable answers `None` and
/// the caller keeps its fallback, which is how a partial store costs one setting
/// rather than the launch.
pub fn Fn<T>(Source:Option<&Value>, Parse:impl Fn(&Value) -> Option<T>) -> Option<Struct<T>> {
	let Source = Source?;

	let Object = Source.as_object();

	Some(match Object {
		Some(Object) => Struct {
			Default:Parse(Object.get("default")?)?,
			Monitor:Object
				.get("monitor")
				.and_then(Value::as_object)
				.map(|Monitor| {
					Monitor
						.iter()
						.filter_map(|(Label, Value)| Parse(Value).map(|Value| (Label.clone(), Value)))
						.collect()
				})
				.unwrap_or_default(),
		},
		None => Build::Fn(Parse(Source)?),
	})
}
