//! A monitor's work area.

use tauri::Monitor;

use crate::Bounds::Struct::Struct;

/// The area of `Monitor` a window may occupy: its rect less the menu bar, the
/// dock or the taskbar.
///
/// Physical pixels exactly as the platform reports them - a work area is an
/// integer rect, so it crosses into `f64` and back without a fractional scale
/// factor ever being rounded. The work area rather than the whole rect is what
/// keeps the tray reachable: a scrim clamped to the full rect would cover the
/// icon that opens and closes the panel.
pub fn Fn(Monitor:&Monitor) -> Struct {
	let Area = Monitor.work_area();

	Struct {
		X:f64::from(Area.position.x),
		Y:f64::from(Area.position.y),
		Width:f64::from(Area.size.width),
		Height:f64::from(Area.size.height),
	}
}
