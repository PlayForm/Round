extern crate winapi;

use winapi::{
	shared::{
		minwindef::{BOOL, LPARAM, TRUE},
		windef::{HDC, HMONITOR, LPRECT},
	},
	um::winuser::{EnumDisplayMonitors, GetMonitorInfoW, MONITORINFOEXW},
};

use std::{io::Error, mem, ptr};

pub fn enumerate_monitors() -> Vec<MONITORINFOEXW> {
	let mut monitors = Vec::<MONITORINFOEXW>::new();
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
		monitors.push(monitor_info);
	}

	TRUE
}
