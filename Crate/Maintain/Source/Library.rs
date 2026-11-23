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
// House style: a two-branch decision is a value, not a branch - the condition
// carries the value (`DryRun.then_some(Mode::Preview).unwrap_or(Mode::Perform)`),
// so a call site reads as one expression instead of a statement with two arms,
// and an ordered precondition reads as `(Cond).then_some(()).ok_or_else(..)`
// rather than an early `return`. That is the opposite of `obfuscated_if_else`'s
// default, which asks for the `if .. else ..` back, so the lint is switched off
// crate-wide rather than re-branching every site.
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
