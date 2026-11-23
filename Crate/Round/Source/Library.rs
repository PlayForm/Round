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
//! * this library - [`Settings::Struct`](Settings::Struct::Struct),
//!   [`Mode::Enum`](Mode::Enum::Enum), [`Overlay`], the
//!   [`Tray::Install`](Tray::Install::Fn) icon, the [`Menu`] panel that drives
//!   them and the [`Backdrop`] it is dismissed by, for anyone embedding the same
//!   overlay in their own Tauri app (`cargo add pround`).
//!
//! House style: module directories and item names are PascalCase, so the
//! snake_case module lint is switched off crate-wide rather than renamed away
//! at every declaration. The tree is the API - a leaf exports one item named by
//! its kind (`Struct`, `Enum`, `Fn`, `Trait`, `Const`) and the directory chain
//! says what that item is for, so `Menu::Place::Fn` is the one function that
//! places the panel and there is no second name for it anywhere.
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
// The Apple platform is `MacOS` as a module, and an acronym is fully cased
// rather than spelled as a word.
#![allow(clippy::upper_case_acronyms)]
// House style: a two-branch decision is a value, not a branch - the condition
// carries the value (`Cond.then(..).unwrap_or(B)`), so the call site reads as one
// expression instead of a statement with two arms. That is the opposite of
// `obfuscated_if_else`'s default, which asks for the `if .. else ..` back, so
// the lint is switched off crate-wide rather than re-branching every site.
#![allow(clippy::obfuscated_if_else)]

pub mod Backdrop;
pub mod Bounds;
pub mod Menu;
pub mod Message;
pub mod Mode;
pub mod Overlay;
pub mod Screen;
pub mod Settings;
pub mod Tray;
pub mod Visibility;
pub mod Window;
