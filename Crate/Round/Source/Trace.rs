//! Debug tracing, compiled out of release builds.
//!
//! The gate is `cfg!(debug_assertions)` inside the macro - a compile-time
//! constant - so a release build dead-codes the format, the stderr write and the
//! file sink. That has to happen *around* the `format!`, not inside the sink:
//! calling a no-op function still leaves the format string in the binary. Mountain
//! gates `dev_log!` the same way and for the same reason.
//!
//! A `tauri dev` build writes one line per event to stderr (the dev terminal) and
//! to the trace file.

// The import is part of the debug build too: without this gate a release build
// warns `unused import`, and CI runs `-D warnings`.
#[cfg(debug_assertions)]
use std::io::Write;

/// Record one line, in a debug build only.
///
/// `Trace!("click set_size label={Label:?}")` - the arguments are `format!`'s,
/// and a release build compiles the whole call away.
#[macro_export]
macro_rules! Trace {
	($($Arg:tt)*) => {
		if cfg!(debug_assertions) {
			$crate::Trace::Fn(&format!($($Arg)*));
		}
	};
}

/// Write one line to stderr and to the trace file. Reached only through the
/// [`Trace!`](crate::Trace!) macro, which is what carries the gate.
#[cfg(debug_assertions)]
pub fn Fn(Message:&str) {
	eprintln!("[round] {Message}");

	let Some(Home) = std::env::var_os("HOME") else {
		return;
	};

	let Path = std::path::Path::new(&Home)
		.join("Library/Application Support/app.roundedcorners.binary")
		.join("trace.log");

	if let Ok(mut File) = std::fs::OpenOptions::new().create(true).append(true).open(Path) {
		let _ = writeln!(File, "{Message}");
	}
}

/// The release build's copy. It exists only so the macro's body resolves; the
/// macro's `cfg!(debug_assertions)` is false there, so the call is dead code and
/// the format string with it.
#[cfg(not(debug_assertions))]
pub fn Fn(_Message:&str) {}
