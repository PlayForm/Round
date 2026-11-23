//! The tray icon, its menu, and what each entry does.

use tauri::{
	AppHandle,
	Runtime,
	menu::{CheckMenuItem, IsMenuItem, Menu, MenuItem, PredefinedMenuItem, Submenu},
	tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
};
use tauri_plugin_store::StoreExt;

use crate::{Action::Action, Effect::Effect, Mode::Mode, Overlay, Settings::Settings};

/// Install the tray icon and wire the menu.
///
/// The menu is built from [`Action`]'s own identifiers and labels, and the
/// handler is one match over [`Effect`] - so a new command is a new enum
/// variant, never a new string in three places.
///
/// The size and ink controls live in submenus because a tray menu is a list,
/// not a form: `Size ▸` keeps the three steps together and leaves the top level
/// for the things you reach for without aiming. Anything that needs a value
/// (a radius per monitor) belongs in the controls window instead.
pub fn install<R:Runtime>(app:&AppHandle<R>, settings:&Settings) -> tauri::Result<()> {
	let item = |action:Action| MenuItem::with_id(app, action.id(), action.label(), true, None::<&str>);

	let check = |action:Action, checked:bool| {
		CheckMenuItem::with_id(app, action.id(), action.label(), true, checked, None::<&str>)
	};

	let increase = item(Action::Increase)?;

	let decrease = item(Action::Decrease)?;

	let reset = item(Action::Reset)?;

	let size = Submenu::with_items(app, "Size", true, &[&increase, &decrease, &reset])?;

	let dark = check(Action::Dark, settings.mode == Mode::Dark)?;

	let light = check(Action::Light, settings.mode == Mode::Light)?;

	let appearance = Submenu::with_items(app, "Appearance", true, &[&dark, &light])?;

	let show = item(Action::Show)?;

	let hide = item(Action::Hide)?;

	let menu_item = item(Action::Menu)?;

	let exit = item(Action::Exit)?;

	let first = PredefinedMenuItem::separator(app)?;

	let second = PredefinedMenuItem::separator(app)?;

	let third = PredefinedMenuItem::separator(app)?;

	#[cfg(target_os = "macos")]
	let dock = CheckMenuItem::with_id(
		app,
		Action::Dock.id(),
		Action::Dock.label(),
		true,
		settings.dock_hidden,
		None::<&str>,
	)?;

	let mut items:Vec<&dyn IsMenuItem<R>> = vec![&size, &appearance, &first, &show, &hide, &second];

	#[cfg(target_os = "macos")]
	{
		items.push(&dock);

		items.push(&third);
	}

	items.push(&menu_item);

	items.push(&exit);

	let menu = Menu::with_items(app, &items)?;

	let dark_item = dark.clone();

	let light_item = light.clone();

	#[cfg(target_os = "macos")]
	let dock_item = dock.clone();

	let _tray = TrayIconBuilder::new()
		.menu(&menu)
		// Left click opens the HTML panel; the native menu stays on right click,
		// where a menu is expected and where the entries that need no value live.
		.show_menu_on_left_click(false)
		.icon_as_template(true)
		.icon(tauri::include_image!("icons/tray.png"))
		.on_tray_icon_event(|tray, event| {
			if let TrayIconEvent::Click {
				button: MouseButton::Left, button_state: MouseButtonState::Up, rect, ..
			} = event
			{
				let _ = crate::Menu::toggle(tray.app_handle(), Some(rect));
			}
		})
		.on_menu_event(move |app, event| {
			let Some(action) = Action::of(event.id.as_ref()) else {
				return;
			};

			let Ok(store) = app.store(Settings::Store) else {
				return;
			};

			let (settings, effect) = Settings::load(&store).next(action);

			let _ = settings.save(&store);

			// The check marks are re-read after every command, not just the one
			// that toggled them, so a mode changed from the controls window can
			// never leave two ticks in the menu.
			let _ = dark_item.set_checked(settings.mode == Mode::Dark);

			let _ = light_item.set_checked(settings.mode == Mode::Light);

			match effect {
				Effect::Exit => app.exit(0),
				Effect::Persist => settings.broadcast(app),
				Effect::Menu => {
					let _ = crate::Menu::toggle(app, None);
				}
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
