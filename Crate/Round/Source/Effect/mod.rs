//! What the caller does once a [`Action`](crate::Action::Action) has been
//! applied to the settings.

/// The follow-up work an action implies.
///
/// A decision, not a flag: the tray handler matches on this instead of
/// re-deriving from the action id what the action was supposed to do.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Effect {
	/// Write the new settings to the store and push them to every window.
	Persist,
	/// Persist, then show or hide the macOS dock icon (`true` = hidden).
	Dock(bool),
	/// Open the controls window.
	Controls,
	/// Quit.
	Exit,
}
