//! Turning a display's name into a window label.

use crate::Screen::Clean;

/// The label a monitor's name asks for.
///
/// A window label may only carry ASCII alphanumerics (Tauri rejects the rest),
/// so "Built-in Retina Display" becomes `BuiltinRetinaDisplay`; a monitor with no
/// name at all becomes `monitor`.
///
/// A name is not an identity: two displays of the same model ask for the same
/// label, and two names can differ only in the punctuation this strips. So this
/// is what one monitor claims, not what it gets - a launch resolves the whole set
/// through [`All`](crate::Screen::All::Fn), which is where a claim becomes a
/// label no other monitor carries.
pub fn Fn(Name:Option<&str>) -> String {
	let Clean = Clean::Fn(Name);

	Clean.is_empty().then(|| "monitor".to_owned()).unwrap_or(Clean)
}
