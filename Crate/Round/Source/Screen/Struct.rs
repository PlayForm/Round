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
/// The other two labels are the ones this display has been stored under before,
/// carried so that a rename costs nothing:
///
/// - `Qualified` is the label it carries while another display shares its name
///   ([`Suffix`](crate::Screen::Suffix::Fn)). Plugging a twin in or pulling one
///   out moves a display between `Label` and `Qualified`, and this is what lets
///   its override follow it in both directions.
/// - `Legacy` is the label the release before uniqueness wrote its overrides
///   under: its name alone, or `monitor<index>` when it had no name at all.
///
/// They are carried rather than recomputed by the reader because only the
/// monitor knows them, and they are what
/// [`Settings::Migrate::Label`](crate::Settings::Migrate::Label::Fn) moves the
/// stored settings off.
pub struct Struct {
	pub Label:String,
	pub Qualified:String,
	pub Legacy:String,
	pub Width:i32,
	pub Height:i32,
	pub X:i32,
	pub Y:i32,
}
