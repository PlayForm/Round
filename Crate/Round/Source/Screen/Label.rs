//! Turning a display's name into a window label.

/// A window label may only carry ASCII alphanumerics (Tauri rejects the rest),
/// so "Built-in Retina Display" becomes `BuiltinRetinaDisplay`; an unnamed
/// monitor becomes `monitor<index>`.
pub fn Fn(Name:Option<&str>, Index:usize) -> String {
	let Clean:String = Name.unwrap_or_default().chars().filter(char::is_ascii_alphanumeric).collect();

	Clean.is_empty().then(|| format!("monitor{Index}")).unwrap_or(Clean)
}
