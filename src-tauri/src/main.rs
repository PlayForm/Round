#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

extern crate regex;
extern crate serde_json;
extern crate tauri;
extern crate tauri_plugin_store;

use std::path::PathBuf;

use regex::Regex;
use serde_json::json;
use tauri::{
	CustomMenuItem, Manager, SystemTray, SystemTrayEvent, SystemTrayMenu, SystemTrayMenuItem,
	WindowBuilder,
};
use tauri_plugin_store::StoreBuilder;

#[derive(Clone, serde::Serialize)]
enum Message {
	Mode(String),
	Size(i64),
}

#[derive(Clone, serde::Serialize)]
struct Payload {
	message: Message,
}

fn main() {
	let default_size = 23;
	let default_mode = "dark";

	tauri::Builder::default()
		.setup(move |app| {
			let mut store =
				StoreBuilder::new(app.app_handle(), PathBuf::from(".settings.dat")).build();

			store.load().unwrap();

			if let None = store.get("size") {
				store.insert("size".to_string(), json!(default_size)).unwrap();
			}

			if let None = store.get("mode") {
				store.insert("mode".to_string(), json!(default_mode)).unwrap();
			}

			store.save().unwrap();
			store.load().unwrap();

			let sample_window =
				WindowBuilder::new(app, "sample", tauri::WindowUrl::App("sample.html".into()))
					.visible(false)
					.always_on_top(false)
					.decorations(false)
					.fullscreen(false)
					.focused(false)
					.title("")
					.position(0.0, 0.0)
					.build()?;

			for monitor in sample_window.available_monitors().unwrap() {
				let label =
					Regex::new(r"[^a-zA-Z0-9\s]").unwrap().replace_all(monitor.name().unwrap(), "");

				let window =
					WindowBuilder::new(app, label, tauri::WindowUrl::App("index.html".into()))
						.always_on_top(true)
						.decorations(false)
						.disable_file_drop_handler()
						.accept_first_mouse(false)
						.focused(false)
						.fullscreen(true)
						.maximized(false)
						.resizable(false)
						.skip_taskbar(true)
						.title("")
						.transparent(true)
						.visible(true)
						.inner_size(monitor.size().width.into(), monitor.size().height.into())
						.position(monitor.position().x.into(), monitor.position().y.into())
						.build()?;

				window.set_cursor_grab(false).unwrap();

				window.show().unwrap();
			}

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
		.on_system_tray_event(move |app, event| {
			let mut store =
				StoreBuilder::new(app.app_handle(), PathBuf::from(".settings.dat")).build();

			store.load().unwrap();

			let mut new_size: i64 = match store.get("size") {
				Some(size) => size.as_i64().unwrap_or(default_size),
				None => default_size,
			};

			let mut new_mode: String = match store.get("mode") {
				Some(mode) => mode.as_str().unwrap_or(default_mode).to_string(),
				None => default_mode.to_string(),
			};

			if let SystemTrayEvent::MenuItemClick { id, .. } = event {
				match id.as_str() {
					"increase" => {
						if let Some(size) = store.get("size") {
							new_size = size.as_i64().unwrap() + 3;
						}
					}
					"decrease" => {
						if let Some(size) = store.get("size") {
							new_size = size.as_i64().unwrap() - 3;
						}
					}
					"reset-size" => {
						new_size = default_size;
					}
					"light" => {
						new_mode = "light".to_string();
					}
					"dark" => {
						new_mode = "dark".to_string();
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

			store.insert("size".to_string(), json!(new_size)).unwrap();
			store.insert("mode".to_string(), json!(new_mode)).unwrap();

			store.save().unwrap();
			store.load().unwrap();

			app.windows().into_iter().for_each(|(_label, window)| {
				if let Some(set_size) = store.get("size") {
					window
						.emit(
							"set-size",
							Payload { message: Message::Size(set_size.as_i64().unwrap()) },
						)
						.unwrap();
				}

				if let Some(set_mode) = store.get("mode") {
					window
						.emit(
							"set-mode",
							Payload {
								message: Message::Mode(set_mode.as_str().unwrap().to_owned()),
							},
						)
						.unwrap();
				}
			});
		})
		.run(tauri::generate_context!())
		.expect("Error! Failed to run Rounded Corners.");
}
