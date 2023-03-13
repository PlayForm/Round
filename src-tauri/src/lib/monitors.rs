use winapi::{
	shared::{
		minwindef::{BOOL, LPARAM, TRUE},
		windef::{HDC, HMONITOR, LPRECT},
	},
	um::{
		wingdi::{GetDeviceCaps, HORZRES, VERTRES},
		winuser::{
			EnumDisplayMonitors, GetMonitorInfoW, GetSystemMetrics, MONITORINFOEXW, SM_CMONITORS,
		},
	},
};

use std::{io::Error, mem, ptr};

pub fn enumerate_monitors() -> Vec<MONITORINFOEXW> {
	let num_monitors = unsafe { GetSystemMetrics(SM_CMONITORS) };
	let mut monitors = Vec::<MONITORINFOEXW>::with_capacity(num_monitors as usize);
	let userdata = &mut monitors as *mut _;

	let result = unsafe {
		EnumDisplayMonitors(
			ptr::null_mut(),
			ptr::null(),
			Some(enumerate_monitors_callback),
			userdata as LPARAM,
		)
	};

	if result != TRUE {
		panic!("Could not enumerate monitors: {}", Error::last_os_error());
	}

	monitors
}

unsafe extern "system" fn enumerate_monitors_callback(
	monitor: HMONITOR,
	_: HDC,
	_: LPRECT,
	userdata: LPARAM,
) -> BOOL {
	let monitors: &mut Vec<MONITORINFOEXW> = mem::transmute(userdata);

	let mut monitor_info: MONITORINFOEXW = mem::zeroed();
	monitor_info.cbSize = mem::size_of::<MONITORINFOEXW>() as u32;
	let monitor_info_ptr = <*mut _>::cast(&mut monitor_info);

	let result = GetMonitorInfoW(monitor, monitor_info_ptr);

	if result == TRUE {
		let hdc = GetDC(ptr::null_mut());
		let horz_res = GetDeviceCaps(hdc, HORZRES);
		let vert_res = GetDeviceCaps(hdc, VERTRES);
		ReleaseDC(ptr::null_mut(), hdc);

		monitor_info.rcMonitor.right = monitor_info.rcMonitor.left + horz_res;
		monitor_info.rcMonitor.bottom = monitor_info.rcMonitor.top + vert_res;

		monitors.push(monitor_info);
	}

	TRUE
}
