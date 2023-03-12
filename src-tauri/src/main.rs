#![cfg_attr(all(not(debug_assertions), target_os = "windows"), windows_subsystem = "windows")]

extern crate screen;
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
	let mut app = tauri::Builder::default()
		.plugin(
			PluginBuilder::default()
				.stores([StoreBuilder::new(".settings.dat").build()])
				.freeze()
				.build(),
		)
		.build(tauri::generate_context!())
		.unwrap();

	let monitors = screen::get_monitors().unwrap();

	for monitor in monitors {
		let label = monitor.name().unwrap_or("Main".to_string());
		let window = app.create_window(label.clone(), tauri::WindowUrl::Blank).unwrap();
		window.set_position(monitor.position().x, monitor.position().y).unwrap();
		window.set_size(monitor.size().width, monitor.size().height).unwrap();
	}

	app.on_system_tray_event(|app, event| {
		if let SystemTrayEvent::MenuItemClick { id, .. } = event {
			match id.as_str() {
				"increase" => {
					app.windows().into_iter().for_each(|(_label, window)| {
						window.emit("switch-size", Payload { message: "increase".into() }).unwrap();
					});
				}
				"decrease" => {
					app.windows().into_iter().for_each(|(_label, window)| {
						window.emit("switch-size", Payload { message: "decrease".into() }).unwrap();
					});
				}
				"reset-size" => {
					app.windows().into_iter().for_each(|(_label, window)| {
						window.emit("switch-size", Payload { message: "reset".into() }).unwrap();
					});
				}
				"light" => {
					app.windows().into_iter().for_each(|(_label, window)| {
						window.emit("switch-mode", Payload { message: "light".into() }).unwrap();
					});
				}
				"dark" => {
					app.windows.into_iter().for_each(|(_label, window)| {
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
	});

	app.run().unwrap();
}
