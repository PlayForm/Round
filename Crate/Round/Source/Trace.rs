//! Debug tracing, gated on the `ROUND_TRACE` environment variable.
//!
//! Nothing is written unless `ROUND_TRACE` is set: a release build carries the
//! calls and produces no output, and `tauri dev` with the variable set gets one
//! line per event on stderr (visible in the dev terminal) and in the trace file.

use std::io::Write;

/// Whether tracing is on for this process.
pub fn On() -> bool { std::env::var_os("ROUND_TRACE").is_some() }

/// Record one line, if tracing is on. Every failure is swallowed - a diagnostic
/// must never be the reason the app misbehaves.
pub fn Fn(Message:&str) {
	if !On() {
		return;
	}

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
