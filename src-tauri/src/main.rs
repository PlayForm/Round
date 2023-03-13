#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

extern crate regex;
extern crate tauri;
extern crate winapi;

use regex::Regex;
use tauri::{
	CustomMenuItem, LogicalSize, Manager, SystemTray, SystemTrayEvent, SystemTrayMenu,
	SystemTrayMenuItem, WindowBuilder,
};

#[derive(Clone, serde::Serialize)]
enum Message {
	String(String),
	Number(i32),
}

#[derive(Clone, serde::Serialize)]
struct Payload {
	message: Message,
}

fn main() {
	tauri::Builder::default()
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
		.setup(|app| {
			let sample_window =
				WindowBuilder::new(app, "sample", tauri::WindowUrl::App("sample.html".into()))
					.build()?;

			for monitor in sample_window.available_monitors().unwrap() {
				let label =
					Regex::new(r"[^a-zA-Z0-9\s]").unwrap().replace_all(monitor.name().unwrap(), "");

				let window =
					WindowBuilder::new(app, label, tauri::WindowUrl::App("index.html".into()))
						.always_on_top(true)
						.decorations(false)
						.disable_file_drop_handler()
						.focused(true)
						.fullscreen(false)
						.maximized(false)
						.resizable(false)
						.skip_taskbar(true)
						.title("")
						.transparent(true)
						.visible(false)
						.build()?;

				window.set_cursor_grab(false).unwrap();
				window
					.set_size(LogicalSize::new(monitor.size().width, monitor.size().height))
					.unwrap();

				// window.set_position(LogicalPosition::new(0, 0)).unwrap();

				window.show().unwrap();
			}

			Ok(())
		})
		.on_system_tray_event(|app, event| {
			if let SystemTrayEvent::MenuItemClick { id, .. } = event {
				match id.as_str() {
					"increase" => {
						app.windows().into_iter().for_each(|(_label, window)| {
							window
								.emit("set-size", Payload { message: Message::Number(23) })
								.unwrap();
						});
					}
					"decrease" => {
						app.windows().into_iter().for_each(|(_label, window)| {
							window
								.emit("set-size", Payload { message: Message::Number(23) })
								.unwrap();
						});
					}
					"reset-size" => {
						app.windows().into_iter().for_each(|(_label, window)| {
							window
								.emit("set-size", Payload { message: Message::Number(23) })
								.unwrap();
						});
					}
					"light" => {
						app.windows().into_iter().for_each(|(_label, window)| {
							window
								.emit(
									"set-mode",
									Payload { message: Message::String("light".to_string()) },
								)
								.unwrap();
						});
					}
					"dark" => {
						app.windows().into_iter().for_each(|(_label, window)| {
							window
								.emit(
									"set-mode",
									Payload { message: Message::String("dark".to_string()) },
								)
								.unwrap();
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
