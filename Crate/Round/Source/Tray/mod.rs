//! The tray icon, and the click that opens the Menu window.
//!
//! The icon carries no menu: every command lives in the [`Menu`](crate::Menu)
//! panel, so the tray is a switch, not a door to a list of items. Nothing here
//! knows what a radius or a mode is - the click is the whole of its behaviour.

use tauri::{
	AppHandle,
	Runtime,
	tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
};

/// Install the tray icon and wire its click to the Menu window.
///
/// The icon is a template image, so macOS paints it in the menu bar's own ink
/// and dark mode needs no second asset.
pub fn install<R:Runtime>(app:&AppHandle<R>) -> tauri::Result<()> {
	let _tray = TrayIconBuilder::new()
		.icon_as_template(true)
		.icon(tauri::include_image!("icons/tray.png"))
		.on_tray_icon_event(|tray, event| {
			// A click counts on the way up: the down half of a press that ends
			// somewhere else, or a button still held, is not a click yet.
			let TrayIconEvent::Click { button, button_state: MouseButtonState::Up, rect, .. } = event else {
				return;
			};

			// Every button is the same switch, because there is no menu behind
			// the icon for a right click to open. The match is exhaustive on
			// purpose: a button the platform grows later has to be wired here
			// rather than quietly ignored.
			match button {
				MouseButton::Left | MouseButton::Right | MouseButton::Middle => {
					if let Err(error) = crate::Menu::toggle(tray.app_handle(), rect) {
						eprintln!("round: the menu window could not be opened: {error}");
					}
				}
			}
		})
		.build(app)?;

	Ok(())
}
