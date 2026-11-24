//! The command surface, in one place.
//!
//! `generate_handler!` produces an unnameable closure, so listing the commands in
//! the binary made the surface untestable: a test could only re-type the list and
//! drift from it. This is the one list - the binary registers it, and a test can
//! register the same one against a mock runtime and invoke a command for real.

use tauri::{ipc::Invoke, Runtime};

use crate::Menu::Command::{
	Benchmark, Dismiss, Dock, Hidden, Mode, Quit, Report, Reset, Size, State,
};

/// Every command the panel and the overlay may call.
pub fn Fn<R:Runtime>() -> impl Fn(Invoke<R>) -> bool + Send + Sync + 'static {
	tauri::generate_handler![
		State::State,
		Size::Size,
		Mode::Mode,
		Hidden::Hidden,
		Dock::Dock,
		Reset::Reset,
		Dismiss::Dismiss,
		Quit::Quit,
		Report::Report,
		Benchmark::Benchmark,
	]
}
