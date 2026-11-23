//! The radius one monitor is drawn with.

use crate::Settings::Struct::Struct;

/// The radius `Label` is drawn with: its own override when it has one, the
/// default otherwise.
pub fn Fn(State:&Struct, Label:&str) -> i64 { State.Sizes.get(Label).copied().unwrap_or(State.Size) }
