//! `pround` - the engine behind Round ⬜.
//!
//! Round draws rounded corners over every connected monitor: one borderless,
//! transparent, click-through webview per screen, each rendering four CSS
//! corners, lifted above the OS chrome by the platform module under
//! [`Overlay`].
//!
//! The crate is both faces of the product:
//!
//! * the `round` binary (`Source/Main.rs`) - the tray application;
//! * this library - [`Settings`](Settings::Settings), [`Mode`](Mode::Mode),
//!   [`Action`](Action::Action) and [`Overlay`], for anyone embedding the same
//!   overlay in their own Tauri app (`cargo add pround`).
//!
//! House style: module directories and item names are PascalCase, so the
//! snake_case module lint is switched off crate-wide rather than renamed away
//! at every declaration.
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
// House style: a two-branch decision is a value, not a branch - the condition
// carries the value (`Cond.then(..).unwrap_or(B)`), so the call site reads as one
// expression instead of a statement with two arms. That is the opposite of
// `obfuscated_if_else`'s default, which asks for the `if .. else ..` back, so
// the lint is switched off crate-wide rather than re-branching every site.
#![allow(clippy::obfuscated_if_else)]

pub mod Action;
pub mod Effect;
pub mod Menu;
pub mod Message;
pub mod Mode;
pub mod Overlay;
pub mod Screen;
pub mod Settings;
pub mod Tray;
