#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

//! The `round` binary: one overlay webview per monitor, then the tray icon.

use pround::{Menu, Overlay, Screen::Screen, Settings::Settings, Tray};
use tauri::{WebviewUrl, WebviewWindowBuilder};
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
			Menu::dismiss,
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

				Overlay::art(&window);

				settings.visible(&window)
			})?;

			Tray::install(app.handle())?;

			Ok(())
		})
		.run(tauri::generate_context!())
		.expect("Cannot Round.");
}
