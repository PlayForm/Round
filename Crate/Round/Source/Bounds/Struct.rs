//! A rectangle in physical pixels.

/// A rectangle in physical pixels, as `f64`.
///
/// Physical rather than logical, and `f64` rather than the integer the OS
/// reports: placement is centring and clamping arithmetic on a panel whose own
/// width is fractional after a scale factor, and rounding twice is how a panel
/// ends up half a pixel off the icon it hangs under. The integer forms are one
/// conversion away in [`Position`](crate::Bounds::Position) and
/// [`Size`](crate::Bounds::Size).
pub struct Struct {
	pub X:f64,
	pub Y:f64,
	pub Width:f64,
	pub Height:f64,
}
