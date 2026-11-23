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
