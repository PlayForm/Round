//! A step as the line a human would type.

use crate::Release::Step::Struct::Struct as Step;

pub fn Fn(Step:&Step) -> String { format!("$ {} {}", Step.Program, Step.Args.join(" ")) }
