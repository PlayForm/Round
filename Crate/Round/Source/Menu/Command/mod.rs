// Debug-only commands: declared only in a debug build, so a release build does
// not compile them and cannot be asked to run them.
#[cfg(debug_assertions)]
pub mod Benchmark;
pub mod Dismiss;
pub mod Dock;
pub mod Hidden;
pub mod Mode;
pub mod Quit;
#[cfg(debug_assertions)]
pub mod Report;
pub mod Reset;
pub mod Size;
pub mod State;
