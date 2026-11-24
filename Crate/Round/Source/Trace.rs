//! TEMPORARY: a line-per-event trace file, for one diagnosis run.
//!
//! The store proves a per-monitor value is written correctly; what is unknown is
//! what each overlay window is labelled and what it is handed. This appends to
//! `~/Library/Application Support/app.roundedcorners.binary/trace.log` so the
//! answer survives however the app is launched. Delete this file and its two
//! call sites once the question is answered.

use std::io::Write;

/// Append one line to the trace file. Every failure is swallowed: a diagnostic
/// must never be the reason the app misbehaves.
pub fn Fn(Message:&str) {
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
