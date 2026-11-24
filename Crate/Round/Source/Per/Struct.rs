//! A default for every monitor, plus the monitors that carry their own value.

use std::collections::BTreeMap;

use serde::Serialize;

/// One setting, as a default and the overrides that depart from it.
///
/// This is the whole of the per-monitor dimension: `Default` is what a monitor
/// takes when it has no entry, and `Monitor` holds the ones tuned on their own,
/// keyed by window label. The radius, the ink and the visibility are each one of
/// these, so the three cannot answer "what does this monitor get" three
/// different ways.
///
/// The store writes this shape as it stands (`{"default": ..., "monitor": {...}}`),
/// so the two keys are pinned here rather than derived from the field names.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize)]
pub struct Struct<T> {
	#[serde(rename = "default")]
	pub Default:T,
	#[serde(rename = "monitor")]
	pub Monitor:BTreeMap<String, T>,
}
