#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

extern crate regex;
extern crate serde_json;
extern crate tauri;
extern crate tauri_plugin_store;

use std::{collections::HashMap, path::PathBuf};

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

#[derive(Debug, PartialEq, Eq, Hash)]
enum KeySettings<'a> {
	Name(&'a str),
}

impl<'a> AsRef<str> for KeySettings<'a> {
	fn as_ref(&self) -> &str {
		match self {
			KeySettings::Name(name) => name,
		}
	}
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
enum ValueSettings<'a> {
	Size(i64),
	Mode(&'a str),
	Hidden(bool),
}

fn main() {
	let mut defaults: HashMap<KeySettings, ValueSettings> = HashMap::new();

	defaults.insert(KeySettings::Name("size"), ValueSettings::Size(23));
	defaults.insert(KeySettings::Name("mode"), ValueSettings::Mode("dark"));
	defaults.insert(KeySettings::Name("hidden"), ValueSettings::Hidden(false));

	tauri::Builder::default()
		.setup(|app| {
			let mut store =
				StoreBuilder::new(app.app_handle(), PathBuf::from("settings.json")).build();

			if let Err(_e) = store.load() {
				store.save().unwrap();
			};

			store.load().unwrap();

			for (key, value) in defaults {
				if let None = store.get(key.as_ref()) {
					store
						.insert(
							key.as_ref().to_owned(),
							json!(json!(value)
								.as_object()
								.and_then(|object| object.values().next())),
						)
						.unwrap();
				}
			}

			store.save().unwrap();
			store.load().unwrap();

			let mut init_script = String::from(r#"window.settings = {};"#);

			if let Some(size) = store.get("size") {
				init_script = init_script + &format!(r#"window.settings.size = {};"#, size);
			}

			if let Some(mode) = store.get("mode") {
				init_script = init_script + &format!(r#"window.settings.mode = {};"#, mode);
			}

			let sample_window =
				WindowBuilder::new(app, "sample", tauri::WindowUrl::App("sample.html".into()))
					.visible(false)
					.always_on_top(false)
					.decorations(false)
					.fullscreen(false)
					.focused(false)
					.title("")
					.position(0.0, 0.0)
					.build()
					.expect("Error! Failed to create a sample window.");

			for monitor in sample_window.available_monitors().unwrap() {
				let label =
					Regex::new(r"[^a-zA-Z0-9\s]").unwrap().replace_all(monitor.name().unwrap(), "");

				let window =
					WindowBuilder::new(app, label, tauri::WindowUrl::App("index.html".into()))
						.always_on_top(true)
						.decorations(false)
						.disable_file_drop_handler()
						.accept_first_mouse(false)
						.focused(true)
						.fullscreen(false)
						.maximized(false)
						.resizable(false)
						.skip_taskbar(true)
						.title("")
						.transparent(true)
						.visible(false)
						.inner_size(monitor.size().width.into(), monitor.size().height.into())
						.position(monitor.position().x.into(), monitor.position().y.into())
						.initialization_script(&init_script)
						.build()
						.expect("Error! Failed to create a window.");

				window.set_cursor_grab(false).unwrap();

				if let Some(hidden) = store.get("hidden") {
					if hidden != true {
						window.show().unwrap();
					}
				}
			}

			sample_window.hide().unwrap();
			sample_window.close().unwrap();

			Ok(())
		})
		.system_tray(
			SystemTray::new().with_menu(
				SystemTrayMenu::new()
					.add_item(CustomMenuItem::new("increase", "➕ Increase Size"))
					.add_item(CustomMenuItem::new("decrease", "➖ Decrease Size"))
					.add_item(CustomMenuItem::new("reset", "↩️ Reset"))
					.add_native_item(SystemTrayMenuItem::Separator)
					.add_item(CustomMenuItem::new("dark", "🌑 Dark"))
					.add_item(CustomMenuItem::new("light", "☀️ Light"))
					.add_native_item(SystemTrayMenuItem::Separator)
					.add_item(CustomMenuItem::new("show", "👨🏻 Show"))
					.add_item(CustomMenuItem::new("hide", "🥷🏽 Hide"))
					.add_item(CustomMenuItem::new("exit", "❌ Exit")),
			),
		)
		.on_system_tray_event(|app, event| {
			let mut store =
				StoreBuilder::new(app.app_handle(), PathBuf::from("settings.json")).build();

			store.load().unwrap();

			let mut size = match store.get("size") {
				Some(size) => size.as_i64().unwrap_or(23),
				None => 23,
			};

			let mut mode = match store.get("mode") {
				Some(mode) => mode.as_str().unwrap_or("dark").to_string(),
				None => "dark".to_string(),
			};

			let mut hidden = match store.get("hidden") {
				Some(hidden) => hidden.as_bool().unwrap_or(false),
				None => false,
			};

			if let SystemTrayEvent::MenuItemClick { id, .. } = event {
				match id.as_str() {
					"increase" => size += 6,
					"decrease" => size -= 6,
					"reset" => size = 23,
					"light" => mode = "light".to_string(),
					"dark" => mode = "dark".to_string(),
					"show" => hidden = false,
					"hide" => hidden = true,
					"exit" => std::process::exit(0),
					_ => {}
				}

				store.insert("size".to_string(), json!(size)).unwrap();
				store.insert("mode".to_string(), json!(mode)).unwrap();
				store.insert("hidden".to_string(), json!(hidden)).unwrap();

				store.save().unwrap();

				app.windows().into_iter().for_each(|(_label, window)| {
					if let Some(size) = store.get("size") {
						window
							.emit(
								"size",
								Payload { message: Message::Size(size.as_i64().unwrap()) },
							)
							.unwrap();
					}

					if let Some(mode) = store.get("mode") {
						window
							.emit(
								"mode",
								Payload {
									message: Message::Mode(mode.as_str().unwrap().to_owned()),
								},
							)
							.unwrap();
					}

					if let Some(hidden) = store.get("hidden") {
						if hidden == true {
							window.hide().unwrap();
						} else {
							window.show().unwrap();
						}
					}
				});
			}
		})
		.run(tauri::generate_context!())
		.expect("Error! Failed to run Rounded Corners.");
}
