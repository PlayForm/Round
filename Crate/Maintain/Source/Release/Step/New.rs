//! Building a step from the literals a ceremony names.

use crate::Release::Step::Struct::Struct as Step;

pub fn Fn(Program:&'static str, Args:&[&str]) -> Step {
	Step { Program, Args:Args.iter().map(|Arg| (*Arg).to_owned()).collect() }
}
