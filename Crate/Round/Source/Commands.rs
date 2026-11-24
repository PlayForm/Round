//! The command surface, in one place.
//!
//! A macro rather than a function, and that is not a style choice: `generate_handler!`
//! expands to a closure that names its runtime, and a generic `fn Fn<R:Runtime>()`
//! cannot satisfy `CommandArg` for an `R` the macro has never seen - it falls back
//! to asking `AppHandle` to deserialize itself. Expanding at the call site keeps a
//! single list while letting each caller pick its own runtime: `Wry` in the binary,
//! `MockRuntime` in a test.

/// Every command the panel and the overlay may call.
#[macro_export]
macro_rules! Commands {
	() => {
		tauri::generate_handler![
			$crate::Menu::Command::State::State,
			$crate::Menu::Command::Size::Size,
			$crate::Menu::Command::Mode::Mode,
			$crate::Menu::Command::Hidden::Hidden,
			$crate::Menu::Command::Dock::Dock,
			$crate::Menu::Command::Reset::Reset,
			$crate::Menu::Command::Dismiss::Dismiss,
			$crate::Menu::Command::Quit::Quit,
			$crate::Menu::Command::Report::Report,
			$crate::Menu::Command::Benchmark::Benchmark,
		]
	};
}
