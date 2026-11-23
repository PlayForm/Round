//! The window ladder: which of Round's three layers sits over which.
//!
//! Three layers, bottom to top:
//!
//! 1. **Art** - the corner overlay, one window per monitor. Click-through, so
//!    a click on it is a click on whatever is under it.
//! 2. **Scrim** - the backdrop the panel is opened over. It takes the mouse:
//!    a click that lands on it is the dismissal.
//! 3. **Panel** - the Menu window, above the scrim, owning its own clicks.
//!
//! Each platform gets its own module and exactly one
//! `art`/`scrim`/`panel`/`front`/`dock` set is re-exported, so call sites read
//! the same everywhere and there is no runtime branch to get wrong.

#[cfg(target_os = "macos")]
mod MacOS;
#[cfg(target_os = "windows")]
mod Windows;
#[cfg(not(any(target_os = "macos", target_os = "windows")))]
mod Linux;

#[cfg(target_os = "macos")]
pub use MacOS::{art, dock, front, panel, scrim};
#[cfg(target_os = "windows")]
pub use Windows::{art, dock, front, panel, scrim};
#[cfg(not(any(target_os = "macos", target_os = "windows")))]
pub use Linux::{art, dock, front, panel, scrim};
