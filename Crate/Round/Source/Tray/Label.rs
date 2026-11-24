//! The tray icon's id.

/// The tray icon's id.
///
/// Named rather than left to the library's generated one: the panel closes by
/// looking the icon up again (`app.tray_by_id`), and a generated id - the process
/// id and a counter - is not something the close path could ever spell.
pub const Const:&str = "round";
