#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
// House style: locals and parameters are PascalCase, the same as the library's.
#![allow(non_snake_case)]

//! The `round` binary: one overlay webview per monitor, then the tray icon.

use pround::{
	Menu::Command::{Dismiss, Dock, Hidden, Mode, Quit, Reset, Size, State},
	Overlay::{Layer::Trait::Trait as _, Platform::Struct as Platform},
	Screen::Of as ScreenOf,
	Settings::{Load as SettingsLoad, Save as SettingsSave, Script, Store::Const as StoreFile, Visible},
	Tray::Install,
	Window::Chrome,
};
use tauri::{WebviewUrl, WebviewWindowBuilder};
use tauri_plugin_store::StoreExt;

fn main() {
	tauri::Builder::default()
		.plugin(tauri_plugin_store::Builder::new().build())
		.invoke_handler(tauri::generate_handler![
			State::State,
			Size::Size,
			Mode::Mode,
			Hidden::Hidden,
			Dock::Dock,
			Reset::Reset,
			Dismiss::Dismiss,
			Quit::Quit,
		])
		.setup(|App| {
			let Storage = App.store(StoreFile)?;

			// Read, then write back: that round trip is how a fresh install gets
			// its defaults on disk before anything reads them again.
			let Preferences = SettingsLoad::Fn(&Storage);

			SettingsSave::Fn(&Preferences, &Storage)?;

			// macOS: the activation policy is process-wide, so it is applied
			// before any window exists.
			Platform::Dock(Preferences.Dock);

			App.available_monitors()?.iter().enumerate().try_for_each(|(Index, Device)| {
				let Screen = ScreenOf::Fn(Device, Index);

				// Per monitor: the script carries the radius THIS window should
				// draw, so a tuned monitor paints correctly before any event.
				let Script = Script::Fn(&Preferences, &Screen.Label);

				let Builder = Chrome::Fn(
					WebviewWindowBuilder::new(App, &Screen.Label, WebviewUrl::App("index.html".into())),
					"",
				)
				.accept_first_mouse(false)
				.focused(false)
				.fullscreen(false)
				.initialization_script(&Script)
				.inner_size(Screen.Width.into(), Screen.Height.into())
				.maximized(false)
				.position(Screen.X.into(), Screen.Y.into())
				.disable_drag_drop_handler();

				// Windows draws its own shadow under a transparent window, which
				// is a grey halo around corners that are meant to be the only
				// edge on screen.
				#[cfg(target_os = "windows")]
				let Builder = Builder.shadow(false);

				let Window = Builder.build()?;

				Platform::Art(&Window);

				Visible::Fn(&Preferences, &Window)
			})?;

			Install::Fn(App.handle())?;

			Ok(())
		})
		.run(tauri::generate_context!())
		.expect("Cannot Round.");
}
