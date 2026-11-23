//! Where the panel goes.

use tauri::{AppHandle, Rect, Runtime, WebviewWindow};

use crate::{
	Bounds::{Monitor as Area, Outer, Position, Rect as AnchorBounds, Struct::Struct as Bounds},
	Menu::{Gap, Height, Width},
};

/// Move the panel under `Anchor`, or above it when there is no room below.
///
/// The screen the icon lives on, not the primary one: an icon on a second
/// display must not drag the panel back to the first. The edges are the screen's
/// work area rather than its whole rect, because the panel is one level above
/// the menu bar: clamped to the full rect, a panel with no room under the icon
/// would be placed over the menu bar and hide the tray icon it was opened from,
/// which is the one control that has to stay reachable while the panel is open.
///
/// Placement is a nicety - a panel in the wrong place is still a panel - so a
/// failure here never aborts the show.
pub fn Fn<R:Runtime>(App:&AppHandle<R>, Window:&WebviewWindow<R>, Anchor:Rect) {
	let Scale = Window.scale_factor().unwrap_or(1.0);

	let Icon = AnchorBounds::Fn(&Anchor, Scale);

	// The size the webview has measured for itself, or the size it was built
	// with before it has measured anything.
	let Panel =
		Outer::Fn(Window).unwrap_or(Bounds { X:0.0, Y:0.0, Width:Width::Const * Scale, Height:Height::Const * Scale });

	// No monitor under the icon is no bound at all: `f64::MAX` is the sentinel
	// that makes both clamps below choose the icon's own side.
	let Area = App
		.monitor_from_point(Icon.X, Icon.Y)
		.ok()
		.flatten()
		.map_or(Bounds { X:0.0, Y:0.0, Width:f64::MAX, Height:f64::MAX }, |Monitor| {
			Area::Fn(&Monitor)
		});

	let Left =
		(Icon.X + Icon.Width / 2.0 - Panel.Width / 2.0).clamp(Area.X, (Area.X + Area.Width - Panel.Width).max(Area.X));

	// Under the icon when there is room, above it otherwise - a tray at the
	// bottom of the screen (Windows) would push the panel off it.
	let Below = Icon.Y + Icon.Height + Gap::Const;

	let Top = (Below + Panel.Height <= Area.Y + Area.Height)
		.then_some(Below)
		.unwrap_or_else(|| (Icon.Y - Panel.Height - Gap::Const).max(Area.Y));

	let _ = Window.set_position(Position::Fn(Left, Top));
}
