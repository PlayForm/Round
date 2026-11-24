//! Pushing the state to every overlay window.

use tauri::{AppHandle, Emitter, Manager, Runtime};

use crate::{
	Backdrop::Owns,
	Menu::Label as MenuLabel,
	Message::{Enum::Enum as Message, Payload::Struct as Payload},
	Per::Of as PerOf,
	Settings::Struct::Struct,
	Visibility::Apply,
};

/// Push the current state to every overlay window.
///
/// Every window is resolved by its own label, so a monitor keeps its own radius,
/// its own ink and its own visibility rather than taking the defaults. Only the
/// art is an overlay. The Menu panel renders the state the command returns and
/// needs no event, and the scrim has no state at all: applying this visibility to
/// it would put a click-eating window over the desktop with the panel closed,
/// once per slider step.
pub fn Fn<R:Runtime>(State:&Struct, App:&AppHandle<R>) {
	App.webview_windows()
		.into_values()
		.filter(|Window| Window.label() != MenuLabel::Const && !Owns::Fn(Window.label()))
		.for_each(|Window| {
			let Label = Window.label().to_owned();

			// TEMPORARY: what this window is called and what it is being handed.
			crate::Trace::Fn(&format!(
				"broadcast label={Label} radius={:?} ink={:?} visible={:?}",
				PerOf::Fn(&State.Radius, &Label),
				PerOf::Fn(&State.Ink, &Label),
				PerOf::Fn(&State.Visible, &Label)
			));

			let _ = Window.emit("size", Payload { Message:Message::Size(PerOf::Fn(&State.Radius, &Label)) });

			let _ = Window.emit("mode", Payload { Message:Message::Mode(PerOf::Fn(&State.Ink, &Label)) });

			let _ = Apply::Fn(PerOf::Fn(&State.Visible, &Label), &Window);
		});
}
