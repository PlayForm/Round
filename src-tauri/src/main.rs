#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

extern crate regex;
extern crate serde_json;
extern crate tauri;
extern crate tauri_plugin_store;

use regex::Regex;
use serde_json::json;
use std::{collections::HashMap, path::PathBuf};
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
				store.save().expect("Error! Could not initialize settings.");
			};

			store.load().expect("Error! Could not get settings.");

			for (key, value) in defaults {
				if let None = store.get(key.as_ref()) {
					store
						.insert(
							key.as_ref().to_owned(),
							json!(json!(value)
								.as_object()
								.and_then(|object| object.values().next())),
						)
						.expect("Error! Could not set defaults.");
				}
			}

			store.save().expect("Error! Could not save settings.");
			store.load().expect("Error! Could not get settings.");

			let mut init_script = String::from(r#"window.settings = {};"#);

			if let Some(size) = store.get("size") {
				init_script = init_script + &format!(r#"window.settings.size = {};"#, size);
			}

			if let Some(mode) = store.get("mode") {
				init_script = init_script + &format!(r#"window.settings.mode = {};"#, mode);
			}

			let sample_window = WindowBuilder::new(
				app,
				"sample",
				tauri::WindowUrl::External("https://roundedcorners.app".parse().unwrap()),
			)
			.visible(false)
			.always_on_top(false)
			.decorations(false)
			.fullscreen(false)
			.focused(false)
			.title("")
			.position(0.0, 0.0)
			.build()
			.expect("Error! Failed to create a sample window.");

			let scale_factor: f64 = sample_window
				.primary_monitor()
				.expect("Error! No monitors found.")
				.expect("Error! Could not get primary monitor.")
				.scale_factor();

			for monitor in
				sample_window.available_monitors().expect("Error! Failed to get monitors.")
			{
				let label_monitor = Regex::new(r"[^a-zA-Z0-9\s]")
					.unwrap()
					.replace_all(monitor.name().expect("Error! Could not get monitor name."), "");

				let monitor_size = monitor.size().to_logical::<i32>(scale_factor);
				let monitor_position = monitor.position().to_logical::<i32>(scale_factor);

				let window = WindowBuilder::new(
					app,
					label_monitor,
					tauri::WindowUrl::App("index.html".into()),
				)
				.always_on_top(true)
				.decorations(false)
				.disable_file_drop_handler()
				.accept_first_mouse(false)
				.focused(false)
				.fullscreen(false)
				.maximized(false)
				.resizable(false)
				.skip_taskbar(true)
				.title("")
				.center()
				.transparent(true)
				.visible(false)
				.inner_size(monitor_size.width.into(), monitor_size.height.into())
				.position(monitor_position.x.into(), monitor_position.y.into())
				.initialization_script(&init_script)
				.build()
				.expect("Error! Failed to create a window.");

				window.set_cursor_grab(false).expect("Error! Could not set cursor grab.");

				if let Some(hidden) = store.get("hidden") {
					if hidden != true {
						window.show().expect("Error! Could not show window");
					}
				}
			}

			sample_window.hide().expect("Error! Could not hide sample window");
			sample_window.close().expect("Error! Could not close sample window");

			Ok(())
		})
		.system_tray(
			SystemTray::new().with_menu(
				SystemTrayMenu::new()
					.add_item(CustomMenuItem::new("increase", "➕ Increase Size"))
					.add_item(CustomMenuItem::new("decrease", "➖ Decrease Size"))
					.add_item(CustomMenuItem::new("reset", "↩️ Reset"))
					.add_native_item(SystemTrayMenuItem::Separator)
					.add_item(CustomMenuItem::new("dark", "🌑 Dark"))
					.add_item(CustomMenuItem::new("light", "☀️ Light"))
					.add_native_item(SystemTrayMenuItem::Separator)
					.add_item(CustomMenuItem::new("show", "👨🏻 Show"))
					.add_item(CustomMenuItem::new("hide", "🥷🏽 Hide"))
					.add_item(CustomMenuItem::new("exit", "❌ Exit")),
			),
		)
		.on_system_tray_event(|app, event| {
			let mut store =
				StoreBuilder::new(app.app_handle(), PathBuf::from("settings.json")).build();

			store.load().expect("Error! Could not get settings.");

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

				store
					.insert("size".to_string(), json!(size))
					.expect("Error! Could not preserve size.");
				store
					.insert("mode".to_string(), json!(mode))
					.expect("Error! Could not preserve mode.");
				store
					.insert("hidden".to_string(), json!(hidden))
					.expect("Error! Could not preserve display.");

				store.save().expect("Error! Could not save settings.");

				app.windows().into_iter().for_each(|(_label, window)| {
					if let Some(size) = store.get("size") {
						window
							.emit(
								"size",
								Payload {
									message: Message::Size(
										size.as_i64()
											.expect("Error! Could not get size from settings."),
									),
								},
							)
							.expect("Error! Could not set size to window.");
					}

					if let Some(mode) = store.get("mode") {
						window
							.emit(
								"mode",
								Payload {
									message: Message::Mode(
										mode.as_str()
											.expect("Error! Could not get mode from settings.")
											.to_owned(),
									),
								},
							)
							.expect("Error! Could not set mode to window.");
					}

					if let Some(hidden) = store.get("hidden") {
						if hidden == true {
							window.hide().expect("Error! Could not hide windows.");
						} else {
							window.show().expect("Error! Could not show windows.");
						}
					}
				});
			}
		})
		.run(tauri::generate_context!())
		.expect("Cannot Round.");
}
