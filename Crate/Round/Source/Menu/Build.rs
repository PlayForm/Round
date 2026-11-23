//! Building the panel window.

use tauri::{AppHandle, Runtime, WebviewUrl, WebviewWindow, WebviewWindowBuilder};

use crate::{
	Menu::Label,
	Overlay::{Layer::Trait::Trait as _, Platform::Struct as Platform},
	Window::Chrome,
};

/// The panel window, built hidden and ordered above the scrims.
///
/// Built once and re-used: `Toggle` builds it on the first click, so a launch
/// that never opens the panel never pays for it.
pub fn Fn<R:Runtime>(App:&AppHandle<R>) -> tauri::Result<WebviewWindow<R>> {
	let Window = Chrome::Fn(
		WebviewWindowBuilder::new(App, Label::Const, WebviewUrl::App("menu.html".into())),
		"Round",
	)
	.shadow(true)
	.build()?;

	Platform::Panel(&Window);

	Ok(Window)
}
