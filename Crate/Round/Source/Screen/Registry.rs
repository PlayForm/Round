//! The monitor set this launch resolved, kept for every command to read.
//!
//! Resolved once, when the overlay windows are built, and read afterwards by
//! every command. That is not an optimisation. A label is unique only within a
//! set, so re-enumerating per command could resolve a label the windows do not
//! carry - which is exactly the drift that let a per-monitor write land on a key
//! no window read. One set, resolved once, is the invariant.
//!
//! It also makes the command layer testable: a mock runtime has no displays, so a
//! test seeds this instead.

use crate::Screen::Struct::Struct as Screen;

/// The set every command reads a monitor's label from.
#[derive(Clone, Default)]
pub struct Struct(pub Vec<Screen>);
