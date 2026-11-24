//! Debug tracing, compiled out of release builds.
//!
//! The gate is `cfg(debug_assertions)` - a compile-time constant - so a release
//! build keeps the call sites and dead-codes the format, the stderr write and the
//! file sink, the way Mountain's `dev_log!` wraps its body in
//! `if cfg!(debug_assertions)`. A `tauri dev` build writes one line per event to
//! stderr (the dev terminal) and to the trace file.

// The import is part of the debug build too: without this gate a release build
// warns `unused import`, and CI runs `-D warnings`.
#[cfg(debug_assertions)]
use std::io::Write;

/// Record one line. Present in a debug build, absent from a release build.
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

/// The release build's copy: same signature, nothing to do.
#[cfg(not(debug_assertions))]
pub fn Fn(_Message:&str) {}
