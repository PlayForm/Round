#![cfg_attr(all(not(debug_assertions), target_os = "windows"), windows_subsystem = "windows")]

extern crate tauri;
extern crate tauri_plugin_store;

use tauri::{
	CustomMenuItem, Manager, SystemTray, SystemTrayEvent, SystemTrayMenu, SystemTrayMenuItem,
};

use tauri_plugin_store::{PluginBuilder, StoreBuilder};

#[derive(Clone, serde::Serialize)]
struct Payload {
	message: String,
}

fn main() {
	tauri::Builder::default()
		.plugin(
			PluginBuilder::default()
				.stores([StoreBuilder::new(".settings.dat".parse().unwrap()).build()])
				.freeze()
				.build(),
		)
		.system_tray(
			SystemTray::new().with_menu(
				SystemTrayMenu::new()
					.add_item(CustomMenuItem::new("increase".to_string(), "➕ Increase Size"))
					.add_item(CustomMenuItem::new("decrease".to_string(), "➖ Decrease Size"))
					.add_item(CustomMenuItem::new("reset-size".to_string(), "↩️ Reset"))
					.add_native_item(SystemTrayMenuItem::Separator)
					.add_item(CustomMenuItem::new("dark".to_string(), "🌑 Dark"))
					.add_item(CustomMenuItem::new("light".to_string(), "☀️ Light"))
					.add_native_item(SystemTrayMenuItem::Separator)
					.add_item(CustomMenuItem::new("show".to_string(), "👨🏻 Show"))
					.add_item(CustomMenuItem::new("hide".to_string(), "🥷🏽 Hide"))
					.add_item(CustomMenuItem::new("exit".to_string(), "❌ Exit")),
			),
		)
		.on_system_tray_event(|app, event| {
			if let SystemTrayEvent::MenuItemClick { id, .. } = event {
				match id.as_str() {
					"increase" => {
						app.windows().into_iter().for_each(|(_label, window)| {
							window
								.emit("switch-size", Payload { message: "increase".into() })
								.unwrap();
						});
					}
					"decrease" => {
						app.windows().into_iter().for_each(|(_label, window)| {
							window
								.emit("switch-size", Payload { message: "decrease".into() })
								.unwrap();
						});
					}
					"reset-size" => {
						app.windows().into_iter().for_each(|(_label, window)| {
							window
								.emit("switch-size", Payload { message: "reset".into() })
								.unwrap();
						});
					}
					"light" => {
						app.windows().into_iter().for_each(|(_label, window)| {
							window
								.emit("switch-mode", Payload { message: "light".into() })
								.unwrap();
						});
					}
					"dark" => {
						app.windows().into_iter().for_each(|(_label, window)| {
							window.emit("switch-mode", Payload { message: "dark".into() }).unwrap();
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
