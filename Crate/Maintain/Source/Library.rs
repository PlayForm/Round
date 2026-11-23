//! `round-maintain` - the CLI behind Round's release.
//!
//! One binary, four jobs: keep the version identical across every manifest,
//! run the gates CI runs, build the app, and cut the release (bump, commit,
//! tag, push, dispatch the crates.io publish).
//!
//! The split mirrors the repository: [`Version`] is the number, [`Manifest`]
//! knows where it lives, [`Process`] runs the tools, and [`Check`], [`Build`]
//! and [`Release`] are the three ceremonies built on those.
//!
//! House style: module directories and item names are PascalCase, so the
//! snake_case module lint is switched off crate-wide rather than renamed away
//! at every declaration.
#![allow(non_snake_case)]
// House style: constants are PascalCase (`const Unreleased`), the same
// convention as every other item, which is the opposite of the lint's default.
#![allow(non_upper_case_globals)]
// House style in the rest of the tree turns a two-branch decision into a value,
// but this is a command-line tool whose job is to fail loudly and early: a guard
// that returns reads as the guard, while `cond.then(..).ok_or(..)` at every
// precondition hides which check refused. `if` stays where it is the clearest
// statement of an early exit, and nowhere else.
#![allow(clippy::obfuscated_if_else)]

pub mod Build;
pub mod Changelog;
pub mod CLI;
pub mod Check;
pub mod Error;
pub mod Manifest;
pub mod Process;
pub mod Release;
pub mod Root;
pub mod Version;
