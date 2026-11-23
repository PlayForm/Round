#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

//! The `round` binary: one overlay webview per monitor, then the tray icon.

use pround::{Menu, Overlay, Probe, Screen::Screen, Settings::Settings, Tray};
use tauri::{Manager, WebviewUrl, WebviewWindowBuilder};
use tauri_plugin_store::StoreExt;

fn main() {
	tauri::Builder::default()
		.plugin(tauri_plugin_store::Builder::new().build())
		.invoke_handler(tauri::generate_handler![
			Menu::state,
			Menu::set_size,
			Menu::set_mode,
			Menu::set_hidden,
			Menu::set_dock,
			Menu::reset,
			Menu::quit,
		])
		.setup(|app| {
			let store = app.store(Settings::Store)?;

			let settings = Settings::load(&store);

			settings.save(&store)?;

			Overlay::dock(!settings.dock_hidden);

			app.available_monitors()?.iter().enumerate().try_for_each(|(index, monitor)| {
				let screen = Screen::of(monitor, index);

				// Per monitor: the script carries the radius THIS window should
				// draw, so a tuned monitor paints correctly before any event.
				let script = settings.script(&screen.label);

				#[allow(unused_mut)]
				let mut builder = WebviewWindowBuilder::new(app, &screen.label, WebviewUrl::App("index.html".into()))
					.accept_first_mouse(false)
					.always_on_top(true)
					.decorations(false)
					.disable_drag_drop_handler()
					.focused(false)
					.fullscreen(false)
					.initialization_script(&script)
					.inner_size(screen.width.into(), screen.height.into())
					.maximized(false)
					.position(screen.x.into(), screen.y.into())
					.resizable(false)
					.title("")
					.transparent(true)
					.visible(false);

				#[cfg(target_os = "windows")]
				{
					builder = builder.skip_taskbar(true).shadow(false);
				}

				let window = builder.build()?;

				Overlay::raise(&window);

				settings.visible(&window)
			})?;

			Tray::install(app.handle(), &settings)?;

			if std::env::var("ROUND_DIAG").is_ok() {
				let handle = app.handle().clone();

				std::thread::spawn(move || {
					std::thread::sleep(std::time::Duration::from_secs(3));

					let once = handle.clone();

					let _ = handle.run_on_main_thread(move || {
						let _ = Menu::toggle(&once, None);
					});

					std::thread::sleep(std::time::Duration::from_secs(2));

					let once = handle.clone();

					let _ = handle.run_on_main_thread(move || {
						if let Some(window) = once.get_webview_window(Menu::Label) {
							eprintln!(
								"[diag] after 2s visible={:?} key={:?} pos={:?} size={:?}",
								window.is_visible(),
								window.is_focused(),
								window.outer_position(),
								window.outer_size()
							);

							let _ = Menu::toggle(&once, None);

							eprintln!("[diag] after second toggle visible={:?}", window.is_visible());
						} else {
							eprintln!("[diag] no window");
						}
					});

					std::thread::sleep(std::time::Duration::from_millis(500));

					handle.exit(0);
				});
			}

			Ok(())
		})
		.run(tauri::generate_context!())
		.expect("Cannot Round.");
}
