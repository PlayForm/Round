//! The part of a display's name a window label may carry.

/// `Name` without anything Tauri would refuse in a window label.
///
/// Only ASCII alphanumerics survive, so "Built-in Retina Display" becomes
/// `BuiltinRetinaDisplay` and "Studio Display - 1" becomes `StudioDisplay1`. A
/// name made only of punctuation therefore cleans to the empty string, which is
/// how a caller learns the name is no use as a label.
pub fn Fn(Name:Option<&str>) -> String {
	Name.unwrap_or_default().chars().filter(char::is_ascii_alphanumeric).collect()
}
