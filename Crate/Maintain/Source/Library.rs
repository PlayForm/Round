//! `Maintain` - the CLI behind Round's release.
//!
//! One binary, four jobs: keep the version identical across every manifest,
//! run the gates CI runs, build the app, and cut the release (bump, commit,
//! tag, push, dispatch the crates.io publish).
//!
//! The split mirrors the repository: [`Version`] is the number, [`Manifest`]
//! knows where it lives, [`Process`] runs the tools, and [`Check`], [`Build`]
//! and [`Release`] are the three ceremonies built on those. [`CLI`] is the
//! command surface over them, and every failure is an `anyhow::Error` whose
//! context names the operation and the file or command that failed.
//!
//! House style: module directories and item names are PascalCase, so the
//! snake_case module lint is switched off crate-wide rather than renamed away
//! at every declaration. The tree is the API - a leaf exports one item named by
//! its kind (`Struct`, `Enum`, `Fn`, `Trait`, `Const`) and the directory chain
//! says what that item is for, so `Manifest::Current::Fn` is the one function
//! that reads the agreed version and there is no second name for it anywhere.
#![allow(non_snake_case)]
// House style: constants are PascalCase (`Unreleased::Const`), the same
// convention as every other item, which is the opposite of the lint's default.
#![allow(non_upper_case_globals)]
// Acronyms are fully cased rather than spelled as a word: `CLI`, `JSON`.
#![allow(clippy::upper_case_acronyms)]
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
pub mod Manifest;
pub mod Process;
pub mod Release;
pub mod Root;
pub mod Version;
