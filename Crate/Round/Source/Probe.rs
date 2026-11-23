//! TEMPORARY diagnostic probe - not part of the product.
//!
//! The operator's session grants no accessibility permission, so `CGEvent`
//! posting is a no-op. Posting an `NSEvent` into this app's own event queue
//! needs no permission and still travels the real path: NSStatusBarWindow ->
//! tray-icon's mouse handler -> Tauri's `TrayIconEvent` -> `Menu::toggle`.

use std::time::Duration;

#[cfg(target_os = "macos")]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
struct CGPoint {
	x:f64,
	y:f64,
}

#[cfg(target_os = "macos")]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
struct CGSize {
	width:f64,
	height:f64,
}

#[cfg(target_os = "macos")]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
struct CGRect {
	origin:CGPoint,
	size:CGSize,
}

#[cfg(target_os = "macos")]
unsafe impl objc2::encode::Encode for CGPoint {
	const ENCODING:objc2::encode::Encoding =
		objc2::encode::Encoding::Struct("CGPoint", &[f64::ENCODING, f64::ENCODING]);
}

#[cfg(target_os = "macos")]
unsafe impl objc2::encode::Encode for CGSize {
	const ENCODING:objc2::encode::Encoding =
		objc2::encode::Encoding::Struct("CGSize", &[f64::ENCODING, f64::ENCODING]);
}

#[cfg(target_os = "macos")]
unsafe impl objc2::encode::Encode for CGRect {
	const ENCODING:objc2::encode::Encoding =
		objc2::encode::Encoding::Struct("CGRect", &[CGPoint::ENCODING, CGSize::ENCODING]);
}

/// Click the tray icon `times` times, `gap` apart, after an initial delay.
#[cfg(target_os = "macos")]
pub fn arm(times:usize, gap:Duration, delay:Duration) {
	if std::env::var("ROUND_PROBE").is_err() {
		return;
	}

	std::thread::spawn(move || {
		use objc2::{class, msg_send, runtime::AnyObject};
		use std::ffi::CStr;

		std::thread::sleep(delay);

		for round in 0..times {
			unsafe {
				let app:*mut AnyObject = msg_send![class!(NSApplication), sharedApplication];

				let windows:*mut AnyObject = msg_send![app, windows];

				let count:usize = msg_send![windows, count];

				let mut target:Option<(*mut AnyObject, isize, CGRect)> = None;

				for index in 0..count {
					let window:*mut AnyObject = msg_send![windows, objectAtIndex: index];

					let class_name_object:*mut AnyObject = msg_send![window, className];

					let class_name:*const i8 = msg_send![class_name_object, UTF8String];

					let name = CStr::from_ptr(class_name).to_string_lossy().into_owned();

					let frame:CGRect = msg_send![window, frame];

					eprintln!("[probe] window {index}: {name} {frame:?}");

					if name.contains("StatusBar") && target.is_none() {
						let number:isize = msg_send![window, windowNumber];

						target = Some((window, number, frame));
					}
				}

				let Some((window, number, frame)) = target else {
					eprintln!("[probe] round {round}: no status bar window");

					continue;
				};

				let point = CGPoint {
					x:frame.size.width / 2.0,
					y:frame.size.height / 2.0,
				};

				let info:*mut AnyObject = msg_send![class!(NSProcessInfo), processInfo];

				let uptime:f64 = msg_send![info, systemUptime];

				eprintln!("[probe] round {round}: clicking window {number} frame {frame:?} at {point:?}");

				for kind in [1isize, 2isize] {
					let event:*mut AnyObject = msg_send![
						class!(NSEvent),
						mouseEventWithType: kind,
						location: point,
						modifierFlags: 0usize,
						timestamp: uptime,
						windowNumber: number,
						context: std::ptr::null_mut::<AnyObject>(),
						eventNumber: 0isize,
						clickCount: 1isize,
						pressure: 1.0f32
					];

					let _:() = msg_send![app, postEvent: event, atStart: false];

					std::thread::sleep(Duration::from_millis(30));
				}

				let _ = window;
			}

			std::thread::sleep(gap);
		}

		eprintln!("[probe] done");
	});
}

#[cfg(not(target_os = "macos"))]
pub fn arm(_times:usize, _gap:Duration, _delay:Duration) {}
