//! Windows: a topmost tool window per layer.

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

/// The window's `HWND`, or `None` when the window does not exist yet.
fn handle<R:Runtime>(window:&WebviewWindow<R>) -> Option<HWND> {
	let hwnd = window.hwnd().ok()?.0 as HWND;

	(!hwnd.is_null()).then_some(hwnd)
}

/// Lift one art window above the taskbar, out of the taskbar list, out of the
/// alt-tab cycle, and out of the mouse's way. Tauri already made it
/// `WS_EX_LAYERED`; the three styles below are what a click-through overlay
/// adds on top. Windows 11's own corner rounding is switched off so the corners
/// drawn by the webview are the only ones.
pub fn art<R:Runtime>(window:&WebviewWindow<R>) {
	let Some(hwnd) = handle(window) else {
		return;
	};

	unsafe {
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
	}
}

/// Put one scrim in the topmost band, out of the taskbar and the alt-tab cycle,
/// and never activated by the click it takes.
///
/// `WS_EX_NOACTIVATE` is the Windows answer to "must not be focusable for
/// typing": the click arrives, the keyboard does not move. `WS_EX_TRANSPARENT`
/// is deliberately absent - that is what the art has and the scrim must not.
pub fn scrim<R:Runtime>(window:&WebviewWindow<R>) {
	let Some(hwnd) = handle(window) else {
		return;
	};

	unsafe {
		let style = GetWindowLongPtrW(hwnd, GWL_EXSTYLE);

		let extra = (WS_EX_TOOLWINDOW | WS_EX_NOACTIVATE) as isize;

		SetWindowLongPtrW(hwnd, GWL_EXSTYLE, style | extra);

		SetWindowPos(hwnd, HWND_TOPMOST, 0, 0, 0, 0, SWP_NOMOVE | SWP_NOSIZE | SWP_NOACTIVATE);
	}
}

/// Put the panel above every scrim.
///
/// A window created after another one enters the topmost band above it, so the
/// scrim would otherwise sit over the panel. `HWND_TOPMOST` with no size or
/// move is a re-insert at the top of the band, which is the whole of the
/// ordering here - and it is applied after the scrims are up, so the panel is
/// on top however many times the panel has been opened.
pub fn panel<R:Runtime>(window:&WebviewWindow<R>) {
	let Some(hwnd) = handle(window) else {
		return;
	};

	unsafe {
		SetWindowPos(hwnd, HWND_TOPMOST, 0, 0, 0, 0, SWP_NOMOVE | SWP_NOSIZE | SWP_NOACTIVATE);
	}
}

/// Put a window on screen.
///
/// The art and the scrim both carry `WS_EX_NOACTIVATE`, so showing either
/// cannot take the keyboard and the portable call is the whole of it here: the
/// no-keying order-front that macOS needs has no Windows equivalent to ask for.
pub fn front<R:Runtime>(window:&WebviewWindow<R>) -> tauri::Result<()> { window.show() }

/// Windows has no dock icon to hide; the setting exists so the menu and the
/// store keep one shape on every platform.
pub fn dock(_visible:bool) {}
