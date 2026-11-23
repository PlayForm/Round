//! Linux: `always_on_top` and `skip_taskbar` are all the window builder gives.

use tauri::{Runtime, WebviewWindow};

use crate::{Overlay::Layer::Trait::Trait, Visibility::Enum::Enum as Visibility};

/// The raised window.
pub struct Struct;

impl Trait for Struct {
	/// Nothing to do beyond what the window builder already asked for. A
	/// compositor may still draw its own panels over the corners; there is no
	/// in-process API that lifts a window above them on every desktop.
	fn Art<R:Runtime>(_Window:&WebviewWindow<R>) {}

	/// Above the windows below it.
	///
	/// There is no per-layer level to ask for here, so the ordering is the
	/// raise: `set_always_on_top(true)` lifts the window back over the rest, and
	/// the panel does the same after the scrims are up. A compositor that
	/// refuses to stack a client above another leaves the scrim under the panel,
	/// where a click on the panel is still the panel's.
	fn Scrim<R:Runtime>(Window:&WebviewWindow<R>) { let _ = Window.set_always_on_top(true); }

	/// Above every scrim. The raise is what orders it, and it runs after the
	/// scrims are shown.
	fn Panel<R:Runtime>(Window:&WebviewWindow<R>) { let _ = Window.set_always_on_top(true); }

	/// The portable call is all the builder leaves to do; whether the compositor
	/// also gives it the keyboard is not something this backend can refuse.
	fn Front<R:Runtime>(Window:&WebviewWindow<R>) -> tauri::Result<()> { Window.show() }

	/// No dock icon to hide.
	fn Dock(_Visible:Visibility) {}
}
