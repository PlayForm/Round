//! Pushing the state to every overlay window.

use tauri::{AppHandle, Emitter, Manager, Runtime};

use crate::{
	Backdrop::Owns,
	Menu::Label as MenuLabel,
	Message::{Enum::Enum as Message, Payload::Struct as Payload},
	Settings::{Radius, Struct::Struct, Visible},
};

/// Push the current state to every overlay window.
///
/// Only the art is an overlay. The Menu panel renders the state the command
/// returns and needs no event, and the scrim has no state at all: applying this
/// visibility to it would put a click-eating window over the desktop with the
/// panel closed, once per slider step.
pub fn Fn<R:Runtime>(State:&Struct, App:&AppHandle<R>) {
	App.webview_windows()
		.into_values()
		.filter(|Window| Window.label() != MenuLabel::Const && !Owns::Fn(Window.label()))
		.for_each(|Window| {
			let Label = Window.label().to_owned();

			let _ = Window.emit("size", Payload { Message:Message::Size(Radius::Fn(State, &Label)) });

			let _ = Window.emit("mode", Payload { Message:Message::Mode(State.Mode) });

			let _ = Visible::Fn(State, &Window);
		});
}
