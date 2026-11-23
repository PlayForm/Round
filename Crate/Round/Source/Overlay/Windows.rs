//! Windows: a topmost tool window the taskbar and the mouse ignore.

use tauri::{Runtime, WebviewWindow};
use windows_sys::Win32::{
	Foundation::HWND,
	Graphics::Dwm::{DWMWA_WINDOW_CORNER_PREFERENCE, DWMWCP_DONOTROUND, DwmSetWindowAttribute},
	UI::WindowsAndMessaging::{
		GWL_EXSTYLE,
		GetWindowLongPtrW,
		HWND_TOPMOST,
		SWP_NOACTIVATE,
		SWP_NOMOVE,
		SWP_NOSIZE,
		SetWindowLongPtrW,
		SetWindowPos,
		WS_EX_NOACTIVATE,
		WS_EX_TOOLWINDOW,
		WS_EX_TRANSPARENT,
	},
};

/// Lift one overlay window above the taskbar, out of the taskbar list, out of
/// the alt-tab cycle, and out of the mouse's way. Tauri already made it
/// `WS_EX_LAYERED`; the three styles below are what a click-through overlay
/// adds on top. Windows 11's own corner rounding is switched off so the corners
/// drawn by the webview are the only ones.
pub fn raise<R:Runtime>(window:&WebviewWindow<R>) {
	let Ok(handle) = window.hwnd() else {
		return;
	};

	let hwnd = handle.0 as HWND;

	let _ = (!hwnd.is_null()).then(|| unsafe {
		let style = GetWindowLongPtrW(hwnd, GWL_EXSTYLE);

		let extra = (WS_EX_TOOLWINDOW | WS_EX_NOACTIVATE | WS_EX_TRANSPARENT) as isize;

		SetWindowLongPtrW(hwnd, GWL_EXSTYLE, style | extra);

		SetWindowPos(hwnd, HWND_TOPMOST, 0, 0, 0, 0, SWP_NOMOVE | SWP_NOSIZE | SWP_NOACTIVATE);

		let corner:i32 = DWMWCP_DONOTROUND;

		DwmSetWindowAttribute(
			hwnd,
			DWMWA_WINDOW_CORNER_PREFERENCE as u32,
			&corner as *const i32 as *const core::ffi::c_void,
			std::mem::size_of::<i32>() as u32,
		);
	});
}

/// Put one overlay window on screen.
///
/// `WS_EX_NOACTIVATE` above already means showing it cannot take the keyboard,
/// so the portable call is the whole of it here: the no-keying order-front that
/// macOS needs has no Windows equivalent to ask for.
pub fn present<R:Runtime>(window:&WebviewWindow<R>) -> tauri::Result<()> { window.show() }

/// Windows has no dock icon to hide; the setting exists so the menu and the
/// store keep one shape on every platform.
pub fn dock(_visible:bool) {}
