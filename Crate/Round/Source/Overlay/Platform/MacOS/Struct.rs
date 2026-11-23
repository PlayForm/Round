//! macOS: an explicit AppKit level per layer.
//!
//! Ordering by "who was shown last" is a fact about a moment; a level is a fact
//! about the window. Re-showing the scrim - which happens on every open - can
//! never lift it over the panel, and nothing about the ladder depends on the
//! order three calls happen to run in.

use objc2::{class, msg_send, runtime::AnyObject};
use tauri::{Runtime, WebviewWindow};

use crate::{
	Overlay::Layer::Trait::Trait,
	Visibility::{Enum::Enum as Visibility, Of as VisibilityOf},
};

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

/// The AppKit window.
pub struct Struct;

/// The window's AppKit handle, or `None` when the window does not exist yet.
fn handle<R:Runtime>(Window:&WebviewWindow<R>) -> Option<*mut AnyObject> {
	let Pointer = Window.ns_window().ok()? as *mut AnyObject;

	(!Pointer.is_null()).then_some(Pointer)
}

impl Trait for Struct {
	/// Above the menu bar, on every Space, in fullscreen apps too, and ignoring
	/// the mouse entirely.
	fn Art<R:Runtime>(Window:&WebviewWindow<R>) {
		let Some(Native) = handle(Window) else {
			return;
		};

		let Behavior = CanJoinAllSpaces | Stationary | IgnoresCycle | FullScreenAuxiliary;

		unsafe {
			let _:() = msg_send![Native, setLevel: Status];

			let _:() = msg_send![Native, setCollectionBehavior: Behavior];

			let _:() = msg_send![Native, setIgnoresMouseEvents: true];
		}
	}

	/// Above the art, below the panel, and in the mouse's way.
	///
	/// The scrim deliberately does **not** join every Space the way the art
	/// does. The panel does not join them either, so a scrim on all Spaces would
	/// outlive the panel the moment the user switched Space: an invisible window
	/// eating every click with nothing over it. `IgnoresCycle` and `Stationary`
	/// are added to whatever behaviour the window already carries, so ⌘` cannot
	/// land on a scrim, Mission Control does not list one, and the fullscreen
	/// flags the builder set are left alone.
	fn Scrim<R:Runtime>(Window:&WebviewWindow<R>) {
		let Some(Native) = handle(Window) else {
			return;
		};

		unsafe {
			let _:() = msg_send![Native, setLevel: Scrim];

			// The one thing that makes a scrim a scrim: the click is delivered
			// here instead of falling through to the desktop.
			let _:() = msg_send![Native, setIgnoresMouseEvents: false];

			let Behavior:u64 = msg_send![Native, collectionBehavior];

			// `Stationary` keeps the scrim out of Mission Control and
			// `IgnoresCycle` out of ⌘`, so the one window of Round that covers a
			// whole screen never appears in a list the user can pick from.
			let _:() = msg_send![Native, setCollectionBehavior: Behavior | Stationary | IgnoresCycle];
		}
	}

	/// One level above the scrim.
	fn Panel<R:Runtime>(Window:&WebviewWindow<R>) {
		let Some(Native) = handle(Window) else {
			return;
		};

		unsafe {
			let _:() = msg_send![Native, setLevel: Panel];
		}
	}

	/// `orderFrontRegardless` - the same ordering as `show` without the key
	/// change, which is also why it still works while the app is inactive.
	fn Front<R:Runtime>(Window:&WebviewWindow<R>) -> tauri::Result<()> {
		let Some(Native) = handle(Window) else {
			// No AppKit window to order: fall back to the portable call, which
			// is only reached on a window that does not exist yet.
			return Window.show();
		};

		unsafe {
			let _:() = msg_send![Native, orderFrontRegardless];
		}

		// A window that did not come up is a layer with no surface at all, which
		// is worse than a key change: verify the ordering took and fall back to
		// the portable call when it did not. A failed query is assumed to have
		// worked, so the fallback never re-shows a window that is already up.
		match Window.is_visible().unwrap_or(true) {
			true => Ok(()),
			false => Window.show(),
		}
	}

	/// The process-wide activation policy, which is what a dock icon is.
	fn Dock(Visible:Visibility) {
		let Policy = match Visible {
			Visibility::Shown => Regular,
			Visibility::Hidden => Accessory,
		};

		unsafe {
			let Application:*mut AnyObject = msg_send![class!(NSApplication), sharedApplication];

			let _ = (!Application.is_null()).then(|| {
				let _:bool = msg_send![Application, setActivationPolicy: Policy];
			});
		}
	}
}

/// `VisibilityOf` is what the trait takes; this keeps the conversion visible at
/// the top of the file rather than inline in a `match` arm.
#[allow(unused_imports)]
use VisibilityOf as _Unused;
