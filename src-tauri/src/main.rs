#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use regex::Regex;
use serde_json::json;
use tauri::{
	Emitter, Manager, WebviewUrl, WebviewWindowBuilder,
	menu::{Menu, MenuItem, PredefinedMenuItem},
	tray::TrayIconBuilder,
};
use tauri_plugin_store::StoreExt;

#[cfg(target_os = "macos")]
fn raise_window<R: tauri::Runtime>(window: &tauri::WebviewWindow<R>) {
	use objc2::{msg_send, runtime::AnyObject};

	let Ok(ns_window) = window.ns_window() else {
		return;
	};

	let ns_window = ns_window as *mut AnyObject;

	if ns_window.is_null() {
		return;
	}

	const NS_STATUS_WINDOW_LEVEL: isize = 25;

	const NS_WINDOW_COLLECTION_BEHAVIOR_CAN_JOIN_ALL_SPACES: u64 = 1 << 0;

	const NS_WINDOW_COLLECTION_BEHAVIOR_STATIONARY: u64 = 1 << 4;

	const NS_WINDOW_COLLECTION_BEHAVIOR_IGNORES_CYCLE: u64 = 1 << 6;

	const NS_WINDOW_COLLECTION_BEHAVIOR_FULL_SCREEN_AUXILIARY: u64 = 1 << 8;

	let behavior = NS_WINDOW_COLLECTION_BEHAVIOR_CAN_JOIN_ALL_SPACES
		| NS_WINDOW_COLLECTION_BEHAVIOR_STATIONARY
		| NS_WINDOW_COLLECTION_BEHAVIOR_IGNORES_CYCLE
		| NS_WINDOW_COLLECTION_BEHAVIOR_FULL_SCREEN_AUXILIARY;

	unsafe {
		let _: () = msg_send![ns_window, setLevel: NS_STATUS_WINDOW_LEVEL];

		let _: () = msg_send![ns_window, setCollectionBehavior: behavior];

		let _: () = msg_send![ns_window, setIgnoresMouseEvents: true];
	}
}

#[cfg(target_os = "windows")]
fn raise_window<R: tauri::Runtime>(window: &tauri::WebviewWindow<R>) {
	use windows_sys::Win32::{
		Foundation::HWND,
		UI::WindowsAndMessaging::{
			GWL_EXSTYLE, GetWindowLongPtrW, HWND_TOPMOST, SWP_NOACTIVATE, SWP_NOMOVE, SWP_NOSIZE,
			SetWindowLongPtrW, SetWindowPos, WS_EX_NOACTIVATE, WS_EX_TOOLWINDOW, WS_EX_TRANSPARENT,
		},
	};

	let Ok(hwnd) = window.hwnd() else {
		return;
	};

	let hwnd = hwnd.0 as HWND;

	if hwnd.is_null() {
		return;
	}

	unsafe {
		let style = GetWindowLongPtrW(hwnd, GWL_EXSTYLE);

		let extra = (WS_EX_TOOLWINDOW | WS_EX_NOACTIVATE | WS_EX_TRANSPARENT) as isize;

		SetWindowLongPtrW(hwnd, GWL_EXSTYLE, style | extra);

		SetWindowPos(
			hwnd,
			HWND_TOPMOST,
			0,
			0,
			0,
			0,
			SWP_NOMOVE | SWP_NOSIZE | SWP_NOACTIVATE,
		);
	}
}

#[cfg(not(any(target_os = "macos", target_os = "windows")))]
fn raise_window<R: tauri::Runtime>(_window: &tauri::WebviewWindow<R>) {}

#[derive(Clone, serde::Serialize)]
enum Message {
	Mode(String),
	Size(i64),
}

#[derive(Clone, serde::Serialize)]
struct Payload {
	message: Message,
}

const STORE_PATH: &str = "settings.json";
const DEFAULT_SIZE: i64 = 23;
const DEFAULT_MODE: &str = "dark";
const SIZE_STEP: i64 = 6;

fn main() {
	tauri::Builder::default()
		.plugin(tauri_plugin_store::Builder::new().build())
		.setup(|app| {
			let store = app.store(STORE_PATH)?;

			if store.get("size").is_none() {
				store.set("size", json!(DEFAULT_SIZE));
			}
			if store.get("mode").is_none() {
				store.set("mode", json!(DEFAULT_MODE));
			}
			if store.get("hidden").is_none() {
				store.set("hidden", json!(false));
			}
			store.save()?;

			let size = store.get("size").and_then(|value| value.as_i64()).unwrap_or(DEFAULT_SIZE);
			let mode = store
				.get("mode")
				.and_then(|value| value.as_str().map(str::to_owned))
				.unwrap_or_else(|| DEFAULT_MODE.to_owned());
			let hidden = store.get("hidden").and_then(|value| value.as_bool()).unwrap_or(false);

			let init_script = format!(
				"window.settings = {{ size: {size}, mode: {mode} }};",
				size = size,
				mode = serde_json::to_string(&mode).unwrap_or_else(|_| "\"dark\"".into()),
			);

			let primary = app.primary_monitor()?.ok_or("Error! No primary monitor.")?;

			let scale_factor = primary.scale_factor();

			let label_re = Regex::new(r"[^a-zA-Z0-9]").unwrap();

			for (index, monitor) in app.available_monitors()?.into_iter().enumerate() {
				let raw_name = monitor.name().cloned().unwrap_or_default();

				let mut label = label_re.replace_all(&raw_name, "").to_string();

				if label.is_empty() {
					label = format!("monitor{index}");
				}

				let size_logical = monitor.size().to_logical::<i32>(scale_factor);

				let position_logical = monitor.position().to_logical::<i32>(scale_factor);

				let window =
					WebviewWindowBuilder::new(app, &label, WebviewUrl::App("index.html".into()))
						.accept_first_mouse(false)
						.always_on_top(true)
						.decorations(false)
						.disable_drag_drop_handler()
						.focused(false)
						.fullscreen(false)
						.initialization_script(init_script.clone())
						.inner_size(size_logical.width.into(), size_logical.height.into())
						.maximized(false)
						.position(position_logical.x.into(), position_logical.y.into())
						.resizable(false)
						.skip_taskbar(true)
						.title("")
						.transparent(true)
						.visible(false)
						.build()?;

				raise_window(&window);

				if !hidden {
					window.show()?;
				}
			}

			let increase =
				MenuItem::with_id(app, "increase", "Increase Size ➕", true, None::<&str>)?;
			let decrease =
				MenuItem::with_id(app, "decrease", "Decrease Size ➖", true, None::<&str>)?;
			let reset = MenuItem::with_id(app, "reset", "Reset ↩️", true, None::<&str>)?;
			let dark = MenuItem::with_id(app, "dark", "Dark 🌑", true, None::<&str>)?;
			let light = MenuItem::with_id(app, "light", "Light ☀️", true, None::<&str>)?;
			let show = MenuItem::with_id(app, "show", "Show 👨🏻", true, None::<&str>)?;
			let hide = MenuItem::with_id(app, "hide", "Hide 🥷🏽", true, None::<&str>)?;
			let exit = MenuItem::with_id(app, "exit", "Exit ❌", true, None::<&str>)?;
			let separator_one = PredefinedMenuItem::separator(app)?;
			let separator_two = PredefinedMenuItem::separator(app)?;

			let menu = Menu::with_items(
				app,
				&[
					&increase,
					&decrease,
					&reset,
					&separator_one,
					&dark,
					&light,
					&separator_two,
					&show,
					&hide,
					&exit,
				],
			)?;

			let tray_icon = tauri::include_image!("icons/tray.png");

			let tray =
				TrayIconBuilder::new().menu(&menu).icon_as_template(true).icon(tray_icon);

			tray.on_menu_event(move |app, event| {
				let store = match app.store(STORE_PATH) {
					Ok(store) => store,
					Err(_) => return,
				};

				let mut size =
					store.get("size").and_then(|value| value.as_i64()).unwrap_or(DEFAULT_SIZE);

				let mut mode = store
					.get("mode")
					.and_then(|value| value.as_str().map(str::to_owned))
					.unwrap_or_else(|| DEFAULT_MODE.to_owned());

				let mut hidden =
					store.get("hidden").and_then(|value| value.as_bool()).unwrap_or(false);

				match event.id.as_ref() {
					"increase" => size += SIZE_STEP,
					"decrease" => size = (size - SIZE_STEP).max(0),
					"reset" => size = DEFAULT_SIZE,
					"dark" => mode = "dark".to_owned(),
					"light" => mode = "light".to_owned(),
					"show" => hidden = false,
					"hide" => hidden = true,
					"exit" => {
						app.exit(0);

						return;
					}
					_ => return,
				}

				store.set("size", json!(size));
				store.set("mode", json!(&mode));
				store.set("hidden", json!(hidden));
				let _ = store.save();

				for (_label, window) in app.webview_windows() {
					let _ = window.emit("size", Payload { message: Message::Size(size) });

					let _ =
						window.emit("mode", Payload { message: Message::Mode(mode.clone()) });

					if hidden {
						let _ = window.hide();
					} else {
						let _ = window.show();
					}
				}
			})
			.build(app)?;

			Ok(())
		})
		.run(tauri::generate_context!())
		.expect("Cannot Round.");
}
