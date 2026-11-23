//! Lifting a transparent webview above the OS chrome.
//!
//! Each platform gets its own module and exactly one `raise` / `present` /
//! `dock` trio is re-exported, so call sites read the same everywhere and there
//! is no runtime branch to get wrong.

#[cfg(target_os = "macos")]
mod MacOS;
#[cfg(target_os = "windows")]
mod Windows;
#[cfg(not(any(target_os = "macos", target_os = "windows")))]
mod Linux;

#[cfg(target_os = "macos")]
pub use MacOS::{dock, present, raise};
#[cfg(target_os = "windows")]
pub use Windows::{dock, present, raise};
#[cfg(not(any(target_os = "macos", target_os = "windows")))]
pub use Linux::{dock, present, raise};
