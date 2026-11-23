//! macOS: an explicit AppKit level per layer.
//!
//! Ordering by "who was shown last" is a fact about a moment; a level is a fact
//! about the window. Re-showing the scrim - which happens on every open - can
//! never lift it over the panel, and nothing about the ladder depends on the
//! order three calls happen to run in.

use objc2::{class, msg_send, runtime::AnyObject};
use tauri::{Runtime, WebviewWindow};

/// `NSStatusWindowLevel` - above the menu bar and every normal window.
const Status:isize = 25;

/// The scrim: one level above the art, so it is the topmost window of Round a
/// click can reach.
const Scrim:isize = Status + 1;

/// The panel: one level above the scrim, so a click on it never reaches the
/// scrim underneath.
const Panel:isize = Scrim + 1;

/// `NSWindowCollectionBehaviorCanJoinAllSpaces`.
const CanJoinAllSpaces:u64 = 1 << 0;

/// `NSWindowCollectionBehaviorStationary`.
const Stationary:u64 = 1 << 4;

/// `NSWindowCollectionBehaviorIgnoresCycle`.
const IgnoresCycle:u64 = 1 << 6;

/// `NSWindowCollectionBehaviorFullScreenAuxiliary`.
const FullScreenAuxiliary:u64 = 1 << 8;

/// `NSApplicationActivationPolicyRegular` - a dock icon.
const Regular:isize = 0;

/// `NSApplicationActivationPolicyAccessory` - no dock icon, no menu bar.
const Accessory:isize = 1;

/// The window's AppKit handle, or `None` when the window does not exist yet.
fn handle<R:Runtime>(window:&WebviewWindow<R>) -> Option<*mut AnyObject> {
	let pointer = window.ns_window().ok()? as *mut AnyObject;

	(!pointer.is_null()).then_some(pointer)
}

/// Lift one art window above the menu bar, on every Space, in fullscreen apps
/// too, and make it ignore the mouse entirely.
pub fn art<R:Runtime>(window:&WebviewWindow<R>) {
	let Some(ns_window) = handle(window) else {
		return;
	};

	let behavior = CanJoinAllSpaces | Stationary | IgnoresCycle | FullScreenAuxiliary;

	unsafe {
		let _:() = msg_send![ns_window, setLevel: Status];

		let _:() = msg_send![ns_window, setCollectionBehavior: behavior];

		let _:() = msg_send![ns_window, setIgnoresMouseEvents: true];
	}
}

/// Put one scrim above the art, below the panel, and in the mouse's way.
///
/// The scrim deliberately does **not** join every Space the way the art does.
/// The panel does not join them either, so a scrim on all Spaces would outlive
/// the panel the moment the user switched Space: an invisible window eating
/// every click with nothing over it. `IgnoresCycle` is added to whatever
/// behaviour the window already carries, so ⌘` cannot land on a scrim and the
/// fullscreen flags the builder set are left alone.
pub fn scrim<R:Runtime>(window:&WebviewWindow<R>) {
	let Some(ns_window) = handle(window) else {
		return;
	};

	unsafe {
		let _:() = msg_send![ns_window, setLevel: Scrim];

		// The one thing that makes a scrim a scrim: the click is delivered
		// here instead of falling through to the desktop.
		let _:() = msg_send![ns_window, setIgnoresMouseEvents: false];

		let behavior:u64 = msg_send![ns_window, collectionBehavior];

		let _:() = msg_send![ns_window, setCollectionBehavior: behavior | IgnoresCycle];
	}
}

/// Put the panel above the scrim.
pub fn panel<R:Runtime>(window:&WebviewWindow<R>) {
	let Some(ns_window) = handle(window) else {
		return;
	};

	unsafe {
		let _:() = msg_send![ns_window, setLevel: Panel];
	}
}

/// Put a window on screen without giving it the keyboard.
///
/// `window.show()` is not just an order-front: Tauri forwards it to tao's
/// `set_visible(true)`, which calls `makeKeyAndOrderFront:` unconditionally.
/// That makes the window key, so whatever held the keyboard resigns it, and
/// macOS delivers `Focused(false)` to it. Neither the art nor the scrim ever
/// wants the keyboard: `orderFrontRegardless` is the same ordering without the
/// key change, which is also why it still works while the app is inactive,
/// where `makeKeyAndOrderFront:` does nothing at all.
pub fn front<R:Runtime>(window:&WebviewWindow<R>) -> tauri::Result<()> {
	let Some(ns_window) = handle(window) else {
		// No AppKit window to order: fall back to the portable call, which is
		// only reached on a window that does not exist yet.
		return window.show();
	};

	unsafe {
		let _:() = msg_send![ns_window, orderFrontRegardless];
	}

	// A window that did not come up is a layer with no surface at all, which is
	// worse than a key change: verify the ordering took and fall back to the
	// portable call when it did not. A failed query is assumed to have worked,
	// so the fallback never re-shows a window that is already up.
	match window.is_visible().unwrap_or(true) {
		true => Ok(()),
		false => window.show(),
	}
}

/// Show or hide the dock icon for the whole process.
pub fn dock(visible:bool) {
	let policy = match visible {
		true => Regular,
		false => Accessory,
	};

	unsafe {
		let application:*mut AnyObject = msg_send![class!(NSApplication), sharedApplication];

		let _ = (!application.is_null()).then(|| {
			let _:bool = msg_send![application, setActivationPolicy: policy];
		});
	}
}
