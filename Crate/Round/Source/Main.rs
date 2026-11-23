#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

//! The `round` binary: one overlay webview per monitor, then the tray icon.

use pround::{Overlay, Screen::Screen, Settings::Settings, Tray};
use tauri::{WebviewUrl, WebviewWindowBuilder};
use tauri_plugin_store::StoreExt;

fn main() {
	tauri::Builder::default()
		.plugin(tauri_plugin_store::Builder::new().build())
		.setup(|app| {
			let store = app.store(Settings::Store)?;

			let settings = Settings::load(&store);

			settings.save(&store)?;

			Overlay::dock(!settings.dock_hidden);

			let script = settings.script();

			app.available_monitors()?.iter().enumerate().try_for_each(|(index, monitor)| {
				let screen = Screen::of(monitor, index);

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

			Ok(())
		})
		.run(tauri::generate_context!())
		.expect("Cannot Round.");
}
