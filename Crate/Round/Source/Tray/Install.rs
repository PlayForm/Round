//! Installing the tray icon and wiring its click.

use tauri::{
	AppHandle,
	Runtime,
	tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
};

use crate::{
	Menu::Toggle,
	Tray::{Label, Session::Set as SessionSet},
};

/// Install the tray icon and wire its click to the Menu window.
///
/// The icon is a template image, so macOS paints it in the menu bar's own ink
/// and dark mode needs no second asset. It carries no menu: every command lives
/// in the panel, so the tray is a switch, not a door to a list of items.
///
/// Its id is set here rather than generated: the panel closes by looking the
/// icon up again to unpress it, and only a name we chose can be looked up.
pub fn Fn<R:Runtime>(App:&AppHandle<R>) -> tauri::Result<()> {
	let _Tray = TrayIconBuilder::with_id(Label::Const)
		.icon_as_template(true)
		.icon(tauri::include_image!("icons/tray.png"))
		.on_tray_icon_event(|Tray, Event| {
			// A click counts on the way up: the down half of a press that ends
			// somewhere else, or a button still held, is not a click yet.
			let TrayIconEvent::Click { button, button_state: MouseButtonState::Up, rect, .. } = Event else {
				return;
			};

			// Before the toggle, because the toggle can close the panel: a right
			// click on an open panel ends its session in this very call, and the
			// close has to repair the highlight this click left behind.
			SessionSet::Fn(button);

			// Every button is the same switch, because there is no menu behind
			// the icon for a right click to open. The match is exhaustive on
			// purpose: a button the platform grows later has to be wired here
			// rather than quietly ignored.
			match button {
				MouseButton::Left | MouseButton::Right | MouseButton::Middle => {
					let _ = Toggle::Fn(Tray.app_handle(), rect)
						.inspect_err(|Error| eprintln!("round: the menu window could not be opened: {Error}"));
				}
			}
		})
		.build(App)?;

	Ok(())
}
