//! The ladder, one backend per platform, under one name.
//!
//! `#[path]` is what replaces the re-export: the three files are selected by
//! `cfg` and all three are declared as the *same* module, so a call site writes
//! `Overlay::Platform::Art(window)` on every platform. There is no second name
//! for a platform function and therefore no `pub use` to keep in step - the
//! compiler resolves `Platform` to whichever file this target compiled.

pub mod Layer;

#[cfg(target_os = "macos")]
#[path = "Platform/MacOS/Struct.rs"]
pub mod Platform;

#[cfg(target_os = "windows")]
#[path = "Platform/Windows/Struct.rs"]
pub mod Platform;

#[cfg(not(any(target_os = "macos", target_os = "windows")))]
#[path = "Platform/Linux/Struct.rs"]
pub mod Platform;
