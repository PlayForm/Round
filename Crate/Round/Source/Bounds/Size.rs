//! A physical size for `set_size`.

use tauri::PhysicalSize;

/// The whole-pixel size a [`Struct`](crate::Bounds::Struct) names.
pub fn Fn(Width:f64, Height:f64) -> PhysicalSize<u32> { PhysicalSize::new(Width.round() as u32, Height.round() as u32) }
