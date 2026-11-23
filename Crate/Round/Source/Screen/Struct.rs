//! One overlay window per connected monitor.

/// A monitor resolved to the window label and the logical bounds a webview
/// window takes.
///
/// `Label` is what every other module keys on - the store's per-monitor
/// overrides, the scrim windows and the broadcast all address a screen by it.
pub struct Struct {
	pub Label:String,
	pub Width:i32,
	pub Height:i32,
	pub X:i32,
	pub Y:i32,
}
