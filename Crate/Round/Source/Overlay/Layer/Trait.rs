//! The contract every platform backend implements.

use tauri::{Runtime, WebviewWindow};

use crate::Visibility::Enum::Enum as Visibility;

/// The window ladder, stated once and implemented per platform.
///
/// Three layers, bottom to top:
///
/// 1. **Art** - the corner overlay, one window per monitor. Click-through, so a
///    click on it is a click on whatever is under it.
/// 2. **Scrim** - the backdrop the panel is opened over. It takes the mouse: a
///    click that lands on it is the dismissal.
/// 3. **Panel** - the Menu window, above the scrim, owning its own clicks.
///
/// The three backends differ in what the OS lets them say - an AppKit level, a
/// topmost re-insert, a raise - but not in what they promise. The trait is the
/// promise: a backend that forgets an operation does not compile, which three
/// copies of the same five signatures could never have caught.
///
/// It is implemented as associated functions rather than methods because there
/// is one of each per process, not one per window: the ladder is a property of
/// the platform, not of a value.
pub trait Trait {
	/// Lift one art window above the OS chrome and out of the mouse's way.
	fn Art<R:Runtime>(Window:&WebviewWindow<R>);

	/// Put one scrim above the art, below the panel, and in the mouse's way.
	fn Scrim<R:Runtime>(Window:&WebviewWindow<R>);

	/// Put the panel above the scrim.
	fn Panel<R:Runtime>(Window:&WebviewWindow<R>);

	/// Put a window on screen without giving it the keyboard.
	///
	/// Deliberately not `WebviewWindow::show`: on macOS that forwards to
	/// `makeKeyAndOrderFront:`, which makes the window key and pulls focus out
	/// of whatever the user was using. Neither the art nor the scrim has any
	/// use for the keyboard.
	fn Front<R:Runtime>(Window:&WebviewWindow<R>) -> tauri::Result<()>;

	/// Show or hide the dock icon for the whole process.
	fn Dock(Visible:Visibility);
}
