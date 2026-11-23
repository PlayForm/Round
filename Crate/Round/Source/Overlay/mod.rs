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
