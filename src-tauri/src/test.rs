#![allow(non_snake_case)]

#[allow(dead_code)]
pub fn Fn() {
	tokio::runtime::Builder::new_multi_thread()
		.enable_all()
		.build()
		.expect("Cannot build.")
		.block_on(async {
			let Builder = if cfg!(debug_assertions) {
				tauri::Builder::default().plugin(tauri_plugin_devtools::init())
			} else {
				tauri::Builder::default()
			};

			Builder
				.plugin(tauri_plugin_shell::init())
				// TODO: FIX THIS
				// .plugin(tauri_plugin_updater::Builder::new().build())
				.setup(|app| {
					/**
					 * Sampler
					 */
					 let Sampler = tauri::WebviewWindowBuilder::new(
						app,
						"Sampler",
						tauri::WebviewUrl::App("index.html".into()),
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

					/**
					 * Scale
					 */
					let Scale: f64 = Sampler
						.primary_monitor()
						.expect("Cannot primary_monitor.")
						.expect("Cannot primary_monitor.")
						.scale_factor();

					for Monitor in Sampler.available_monitors().expect("Cannot available_monitors.")
					{
						/**
						 * Label, Size & Position
						 */
						let Label = Regex::new(r"[^a-zA-Z0-9\s]")
								.unwrap()
								.replace_all(Monitor.name().expect("Cannot name."), "");
						let SizeMonitor = Monitor.size().to_logical::<i32>(Scale);
						let PositionMonitor = Monitor.position().to_logical::<i32>(Scale);

						/**
						 * Daemon
						 */
						 let Daemon = tauri::WebviewWindowBuilder::new(
							app,
							"Daemon",
							tauri::WebviewUrl::App("index.html".into()),
						)
						.accept_first_mouse(false)
						// .additional_browser_args("")
						.always_on_bottom(false)
						.always_on_top(true)
						.center()
						.closable(false)
						.content_protected(true)
						.decorations(false)
						.drag_and_drop(true)
						.focused(true)
						.fullscreen(true)
						.incognito(true)
						.maximizable(false)
						.maximized(false)
						.minimizable(false)
						.resizable(false)

						
						.shadow(false)
						// .skip_taskbar(true)
						.theme(Some(tauri::Theme::Light))
						.title("")
						.transparent(true)
						.user_agent("")
						.visible(true)
						.visible_on_all_workspaces(true)
						.effects(tauri::utils::config::WindowEffectsConfig {
							..Default::default()
						})
						.zoom_hotkeys_enabled(false)
						.inner_size(SizeMonitor.width.into(), SizeMonitor.height.into())
				.position(PositionMonitor.x.into(), PositionMonitor.y.into())

						.build()
						.expect("Cannot build.");
						
						// Daemon.set_cursor_grab(false).expect("Cannot set_cursor_grab.");
						// Daemon.set_ignore_cursor_events(true).expect("Cannot set_ignore_cursor_events.");
					}

					Ok(())
				})
				.run(tauri::generate_context!())
				.expect("Cannot run.");
		});
}


