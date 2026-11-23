//! One overlay window per connected monitor.

use tauri::Monitor;

/// A monitor resolved to the window label and the logical bounds a webview
/// window takes.
pub struct Screen {
	pub label: String,
	pub width: i32,
	pub height: i32,
	pub x: i32,
	pub y: i32,
}

impl Screen {
	pub fn of(monitor: &Monitor, index: usize) -> Self {
		let factor = monitor.scale_factor();

		let size = monitor.size().to_logical::<i32>(factor);

		let position = monitor.position().to_logical::<i32>(factor);

		Self {
			label: label(monitor.name().map(String::as_str), index),
			width: size.width,
			height: size.height,
			x: position.x,
			y: position.y,
		}
	}
}

/// A window label may only carry ASCII alphanumerics (Tauri rejects the rest),
/// so "Built-in Retina Display" becomes `BuiltinRetinaDisplay`; an unnamed
/// monitor becomes `monitor<index>`.
fn label(name: Option<&str>, index: usize) -> String {
	let clean: String = name.unwrap_or_default().chars().filter(char::is_ascii_alphanumeric).collect();

	clean.is_empty().then(|| format!("monitor{index}")).unwrap_or(clean)
}
