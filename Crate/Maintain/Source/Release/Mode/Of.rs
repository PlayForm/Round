//! Reading the mode off the flag that selects it.

use crate::Release::Mode::Enum::Enum as Mode;

pub fn Fn(DryRun:bool) -> Mode { DryRun.then_some(Mode::Preview).unwrap_or(Mode::Perform) }
