//! What one monitor asks to be called, before the set settles it.

/// The label a monitor's name asks for, and where the monitor sits on the
/// desktop in physical pixels.
///
/// A claim is not a label: two displays of the same model make the same one.
/// The position is carried alongside because it is the only fact that tells such
/// a pair apart - two windows cannot occupy the same place - and because it
/// belongs to the display rather than to the order the system listed it in.
pub struct Struct {
	pub Label:String,
	pub X:i32,
	pub Y:i32,
}
