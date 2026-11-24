#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
// House style: locals and parameters are PascalCase, the same as the library's.
#![allow(non_snake_case)]

//! The `round` binary: one overlay webview per monitor, then the tray icon.

use pround::{
	Overlay::{Layer::Trait::Trait as _, Platform::Struct as Platform},
	Per::Of as PerOf,
	Screen::{All as ScreenAll, Registry},
	Settings::{Load as SettingsLoad, Save as SettingsSave, Script, Store::Const as StoreFile},
	Tray::Install,
	Visibility::Apply as VisibilityApply,
	Window::Chrome,
};
use tauri::{Manager, WebviewUrl, WebviewWindowBuilder};
use tauri_plugin_store::StoreExt;

fn main() {
	tauri::Builder::default()
		.plugin(tauri_plugin_store::Builder::new().build())
		.invoke_handler(pround::Commands!())
		.setup(|App| {
			let Storage = App.store(StoreFile)?;

			// The monitors come before the settings: every override is keyed by
			// the label a monitor is given, and a label is unique only against
			// the rest of the set, so the set is what decides where a stored
			// setting lands.
			let Devices = App.available_monitors()?;

			let Screens = ScreenAll::Fn(&Devices);

			// The one set every command will read a label from. Resolved here,
			// beside the windows it names, so a command can never disagree with
			// them about which monitor carries which label.
			App.manage(Registry::Struct(Screens.clone()));

			// Read, then write back: that round trip is how a fresh install gets
			// its defaults on disk before anything reads them again, and how a
			// store an older build wrote is folded into the per-monitor shape
			// once, at launch, instead of on every command.
			let Preferences = SettingsLoad::Fn(&Storage, &Screens);

			SettingsSave::Fn(&Preferences, &Storage)?;

			// macOS: the activation policy is process-wide, so it is applied
			// before any window exists.
			Platform::Dock(Preferences.Dock);

			Screens.iter().try_for_each(|Screen| {
				pround::Trace::Fn(&format!(
					"window {} radius={:?} ink={:?}",
					Screen.Label,
					pround::Per::Of::Fn(&Preferences.Radius, &Screen.Label),
					pround::Per::Of::Fn(&Preferences.Ink, &Screen.Label)
				));

				// Per monitor: the script carries the radius and ink THIS window
				// should draw, so a tuned monitor paints correctly before any
				// event.
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

				VisibilityApply::Fn(PerOf::Fn(&Preferences.Visible, &Screen.Label), &Window)
			})?;

			Install::Fn(App.handle())?;

			Ok(())
		})
		.run(tauri::generate_context!())
		.expect("Cannot Round.");
}
