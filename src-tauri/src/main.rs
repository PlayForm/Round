#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

extern crate tauri;
extern crate tauri_plugin_store;

use std::{ffi::OsString, os::windows::ffi::OsStringExt};
use tauri::{
	CustomMenuItem, Manager, SystemTray, SystemTrayEvent, SystemTrayMenu, SystemTrayMenuItem,
};
use tauri_plugin_store::{Builder, StoreBuilder};

#[derive(Clone, serde::Serialize)]
struct Payload {
	message: String,
}

fn main() {
	for monitor in monitors::enumerate_monitors() {
		let name = match &monitor.szDevice[..].iter().position(|c| *c == 0) {
			Some(len) => OsString::from_wide(&monitor.szDevice[0..*len]),
			None => OsString::from_wide(&monitor.szDevice[0..monitor.szDevice.len()]),
		};

		println!("Display name = {}", name.to_str().unwrap());
		println!("    Left: {}", monitor.rcWork.left);
		println!("   Right: {}", monitor.rcWork.right);
		println!("     Top: {}", monitor.rcWork.top);
		println!("  Bottom: {}", monitor.rcWork.bottom);
	}

	tauri::Builder::default()
		.plugin(Builder::default().build())
		.setup(|app| {
			let _store = StoreBuilder::new(app.handle(), ".settings.dat".parse()?).build();

			app.windows().into_iter().for_each(|(label, _window)| {
				println!("{:?}", label);
			});

			Ok(())
		})
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
