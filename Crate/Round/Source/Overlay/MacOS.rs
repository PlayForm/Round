//! macOS: a status-level, click-through, all-spaces window.

use objc2::{class, msg_send, runtime::AnyObject};
use tauri::{Runtime, WebviewWindow};

/// `NSStatusWindowLevel` - above the menu bar and every normal window.
const Level:isize = 25;

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

/// Whether a click can reach the window.
///
/// [`raise`] makes the overlay click-through, so a click that lands on its rect
/// is delivered to whatever is under it - the desktop, another application -
/// and is not a click on Round. A window whose AppKit handle cannot be read is
/// reported as clickable, so a caller asking whether the user left keeps the
/// panel open rather than dismissing on missing data.
pub fn clickable<R:Runtime>(window:&WebviewWindow<R>) -> bool {
	let Ok(pointer) = window.ns_window() else {
		return true;
	};

	let ns_window = pointer as *mut AnyObject;

	match ns_window.is_null() {
		true => true,
		false => {
			let ignores:bool = unsafe { msg_send![ns_window, ignoresMouseEvents] };

			!ignores
		}
	}
}

/// Lift one overlay window above the menu bar, on every Space, in fullscreen
/// apps too, and make it ignore the mouse entirely.
pub fn raise<R:Runtime>(window:&WebviewWindow<R>) {
	let Ok(pointer) = window.ns_window() else {
		return;
	};

	let ns_window = pointer as *mut AnyObject;

	let behavior = CanJoinAllSpaces | Stationary | IgnoresCycle | FullScreenAuxiliary;

	let _ = (!ns_window.is_null()).then(|| unsafe {
		let _:() = msg_send![ns_window, setLevel: Level];

		let _:() = msg_send![ns_window, setCollectionBehavior: behavior];

		let _:() = msg_send![ns_window, setIgnoresMouseEvents: true];
	});
}

/// Put one overlay window on screen without giving it the keyboard.
///
/// `window.show()` is not just an order-front: Tauri forwards it to tao's
/// `set_visible(true)`, which calls `makeKeyAndOrderFront:` unconditionally.
/// That makes the overlay the key window, so the Menu panel resigns key, macOS
/// delivers `Focused(false)` to it, and the panel - whose app is still
/// frontmost, so the blur looks exactly like a dismissal - closes itself. An
/// overlay never receives a click, so it has no use for the keyboard:
/// `orderFrontRegardless` is the same ordering without the key change, which is
/// also why it still works while the app is inactive, where
/// `makeKeyAndOrderFront:` does nothing at all.
pub fn present<R:Runtime>(window:&WebviewWindow<R>) -> tauri::Result<()> {
	let pointer = window.ns_window()?;

	let ns_window = pointer as *mut AnyObject;

	match ns_window.is_null() {
		// No AppKit window to order: fall back to the portable call, which is
		// only reached on a window that does not exist yet.
		true => window.show(),
		false => unsafe {
			let _:() = msg_send![ns_window, orderFrontRegardless];

			// A window that did not come up is an overlay with no corners at
			// all, which is worse than a key change: verify the ordering took
			// and fall back to the portable call when it did not. A failed
			// query is assumed to have worked, so the fallback never re-shows a
			// window that is already up.
			match window.is_visible().unwrap_or(true) {
				true => Ok(()),
				false => window.show(),
			}
		},
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
