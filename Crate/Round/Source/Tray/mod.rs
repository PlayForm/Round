//! The tray icon, its menu, and what each entry does.

use tauri::{
	AppHandle,
	Runtime,
	menu::{IsMenuItem, Menu, MenuItem, PredefinedMenuItem},
	tray::TrayIconBuilder,
};
// macOS-only: the dock entry is the only `CheckMenuItem` in the menu. Importing
// it unconditionally is dead code on Linux and Windows, and `-D warnings` in CI
// turns that into a hard failure macOS can never reproduce.
#[cfg(target_os = "macos")]
use tauri::menu::CheckMenuItem;
use tauri_plugin_store::StoreExt;

use crate::{Action::Action, Effect::Effect, Overlay, Settings::Settings};

/// Install the tray icon and wire the menu.
///
/// The menu is built from [`Action`]'s own identifiers and labels, and the
/// handler is one match over [`Effect`] - so a new command is a new enum
/// variant, never a new string in three places.
///
/// `settings` is read only by the macOS dock entry; the parameter exists on
/// every platform so the call site stays identical.
#[cfg_attr(not(target_os = "macos"), allow(unused_variables))]
pub fn install<R:Runtime>(app:&AppHandle<R>, settings:&Settings) -> tauri::Result<()> {
	let item = |action:Action| MenuItem::with_id(app, action.id(), action.label(), true, None::<&str>);

	let increase = item(Action::Increase)?;

	let decrease = item(Action::Decrease)?;

	let reset = item(Action::Reset)?;

	let dark = item(Action::Dark)?;

	let light = item(Action::Light)?;

	let show = item(Action::Show)?;

	let hide = item(Action::Hide)?;

	let exit = item(Action::Exit)?;

	let first = PredefinedMenuItem::separator(app)?;

	let second = PredefinedMenuItem::separator(app)?;

	#[cfg(target_os = "macos")]
	let dock = CheckMenuItem::with_id(
		app,
		Action::Dock.id(),
		Action::Dock.label(),
		true,
		settings.dock_hidden,
		None::<&str>,
	)?;

	#[cfg(target_os = "macos")]
	let third = PredefinedMenuItem::separator(app)?;

	let mut items:Vec<&dyn IsMenuItem<R>> =
		vec![&increase, &decrease, &reset, &first, &dark, &light, &second, &show, &hide];

	#[cfg(target_os = "macos")]
	{
		items.push(&third);

		items.push(&dock);
	}

	items.push(&exit);

	let menu = Menu::with_items(app, &items)?;

	#[cfg(target_os = "macos")]
	let dock_item = dock.clone();

	let _tray = TrayIconBuilder::new()
		.menu(&menu)
		.icon_as_template(true)
		.icon(tauri::include_image!("icons/tray.png"))
		.on_menu_event(move |app, event| {
			let Some(action) = Action::of(event.id.as_ref()) else {
				return;
			};

			let Ok(store) = app.store(Settings::Store) else {
				return;
			};

			let (settings, effect) = Settings::load(&store).next(action);

			let _ = settings.save(&store);

			match effect {
				Effect::Exit => app.exit(0),
				Effect::Persist => settings.broadcast(app),
				Effect::Dock(hidden) => {
					#[cfg(target_os = "macos")]
					let _ = dock_item.set_checked(hidden);

					Overlay::dock(!hidden);

					settings.broadcast(app);
				}
			}
		})
		.build(app)?;

	Ok(())
}
