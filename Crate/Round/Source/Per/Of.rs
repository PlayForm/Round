//! The value one monitor is drawn with.

use crate::Per::Struct::Struct;

/// The value `Label` takes: its own override when it has one, the default
/// otherwise.
///
/// Every reader that wants a value rather than an override goes through here, so
/// falling back is not a step a caller can forget.
pub fn Fn<T:Copy>(State:&Struct<T>, Label:&str) -> T { State.Monitor.get(Label).copied().unwrap_or(State.Default) }
