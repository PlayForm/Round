//! The command surface, in one place.
//!
//! A macro rather than a function, and that is not a style choice: `generate_handler!`
//! expands to a closure that names its runtime, and a generic `fn Fn<R:Runtime>()`
//! cannot satisfy `CommandArg` for an `R` the macro has never seen - it falls back
//! to asking `AppHandle` to deserialize itself. Expanding at the call site keeps a
//! single list while letting each caller pick its own runtime: `Wry` in the binary,
//! `MockRuntime` in a test.
//!
//! Two lists, because `Benchmark` exists only in a debug build. A release build
//! does not merely skip it - it has no such command, so nothing can call it.

/// Every command the panel and the overlay may call, debug build.
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
			$crate::Menu::Command::Benchmark::Benchmark,
		]
	};
}

/// The same surface without the two debug-only commands, release build.
#[macro_export]
macro_rules! CommandsRelease {
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
		]
	};
}
