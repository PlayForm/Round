//! The part of a display's name a window label may carry.

/// `Name` reduced to the characters a label is safe in.
///
/// Only ASCII alphanumerics survive, so "Built-in Retina Display" becomes
/// `BuiltinRetinaDisplay` and "Studio Display - 1" becomes `StudioDisplay1`. A
/// name made only of punctuation therefore cleans to the empty string, which is
/// how a caller learns the name is no use as a label.
///
/// `-`, `_`, `/` and `:` are also accepted in a label, and [`Suffix`] uses two of
/// them; a display's own name is held to the narrower alphabet anyway, so that no
/// label depends on which of those characters a platform happens to allow. That
/// is also why two names can collide: "Studio Display - 1" and "Studio Display 1"
/// are one label before [`Unique`] sees them.
///
/// [`Suffix`]: crate::Screen::Suffix::Fn
/// [`Unique`]: crate::Screen::Unique::Fn
pub fn Fn(Name:Option<&str>) -> String {
	Name.unwrap_or_default().chars().filter(char::is_ascii_alphanumeric).collect()
}
