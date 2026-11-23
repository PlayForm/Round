//! Every command the tray menu can send.

/// One tray command.
///
/// The menu identifier the event carries (`id`) and the text the menu draws
/// (`label`) both live on the variant, so adding a command is one variant plus
/// two arms - never a string repeated across the tray builder, the event
/// handler and the settings update.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Action {
	Increase,
	Decrease,
	Reset,
	Dark,
	Light,
	Show,
	Hide,
	/// macOS only: hide the dock icon while Round is running.
	Dock,
	Exit,
}

impl Action {
	/// Every command, in one place, so [`Action::of`] cannot drift from the
	/// identifiers the menu items were built with.
	pub const Every: [Self; 9] = [
		Self::Increase,
		Self::Decrease,
		Self::Reset,
		Self::Dark,
		Self::Light,
		Self::Show,
		Self::Hide,
		Self::Dock,
		Self::Exit,
	];

	pub const fn id(self) -> &'static str {
		match self {
			Self::Increase => "increase",
			Self::Decrease => "decrease",
			Self::Reset => "reset",
			Self::Dark => "dark",
			Self::Light => "light",
			Self::Show => "show",
			Self::Hide => "hide",
			Self::Dock => "dock",
			Self::Exit => "exit",
		}
	}

	/// Menu text. `\u{2001}` (em quad) separates the words from the glyph so the
	/// two do not collide in a proportional menu font.
	pub const fn label(self) -> &'static str {
		match self {
			Self::Increase => "Increase Size\u{2001}➕",
			Self::Decrease => "Decrease Size\u{2001}➖",
			Self::Reset => "Reset\u{2001}↩️",
			Self::Dark => "Dark\u{2001}🌑",
			Self::Light => "Light\u{2001}☀️",
			Self::Show => "Show\u{2001}👨🏻",
			Self::Hide => "Hide\u{2001}🥷🏽",
			Self::Dock => "No Dock\u{2001}⚓",
			Self::Exit => "Exit\u{2001}❌",
		}
	}

	pub fn of(id: &str) -> Option<Self> {
		Self::Every.into_iter().find(|action| action.id() == id)
	}
}
