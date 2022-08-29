#![cfg_attr(all(not(debug_assertions), target_os = "windows"), windows_subsystem = "windows")]

use tauri::{CustomMenuItem, Manager, SystemTray, SystemTrayEvent, SystemTrayMenu};

fn main() {
	tauri::Builder::default()
		.system_tray(
			SystemTray::new().with_menu(
				SystemTrayMenu::new()
					.add_item(CustomMenuItem::new("dark".to_string(), "Dark"))
					.add_item(CustomMenuItem::new("white".to_string(), "White"))
					.add_item(CustomMenuItem::new("show".to_string(), "Show"))
					.add_item(CustomMenuItem::new("hide".to_string(), "Hide"))
					.add_item(CustomMenuItem::new("exit".to_string(), "Exit")),
			),
		)
		.on_system_tray_event(|app, event| {
			if let SystemTrayEvent::MenuItemClick { id, .. } = event {
				match id.as_str() {
					"dark" => {
						app.windows().into_iter().for_each(|(_label, window)| {
							window.with_webview(move |webview| webview.controller())
						});
					}
					"white" => {
						app.windows().into_iter().for_each(|(_label, window)| {
							window.show().unwrap();
						});
					}
					"show" => {
						app.windows().into_iter().for_each(|(_label, window)| {
							window.show().unwrap();
						});
					}
					"hide" => {
						app.windows().into_iter().for_each(|(_label, window)| {
							window.hide().unwrap();
						});
					}
					"exit" => {
						std::process::exit(0);
					}
					_ => {}
				}
			}
		})
		.run(tauri::generate_context!())
		.expect("Error! Failed to run Rounded Corners.");
}
