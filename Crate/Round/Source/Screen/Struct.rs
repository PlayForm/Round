//! One overlay window per connected monitor.

/// A monitor resolved to the window label and the logical bounds a webview
/// window takes.
///
/// `Label` is what every other module keys on - the store's per-monitor
/// overrides, the scrim windows and the broadcast all address a screen by it -
/// so it is unique within the set: no two monitors in one launch carry the same
/// one, which is what lets two displays of the same name have their own window
/// and their own settings.
///
/// `Legacy` is the label the release before that uniqueness wrote this monitor's
/// overrides under. It is carried rather than recomputed by the reader because
/// only the monitor knows it, and it is what
/// [`Settings::Migrate::Label`](crate::Settings::Migrate::Label::Fn) moves the
/// stored settings off.
pub struct Struct {
	pub Label:String,
	pub Legacy:String,
	pub Width:i32,
	pub Height:i32,
	pub X:i32,
	pub Y:i32,
}
