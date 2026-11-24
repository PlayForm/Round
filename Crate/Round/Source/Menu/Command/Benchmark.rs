//! Debug: pumping the command path to measure it.
//!
//! A benchmark belongs in the debug build only. Both profiles define the command
//! so `generate_handler!` can name it unconditionally, and the release body is a
//! single `Err` - the same `cfg(debug_assertions)` split the trace module uses.

use tauri::{AppHandle, Runtime};

#[cfg(debug_assertions)]
use crate::{
	Menu::Store::{Commit, Load},
	Mode::Of as ModeOf,
	Per::Set as PerSet,
	Settings::Struct::Struct as Settings,
};

/// Run `Count` full write-save-broadcast-state cycles and report the spread.
///
/// Every cycle is the real path a panel click takes, not a synthetic loop: load
/// the store, write one monitor's ink, commit (save + broadcast + rebuild the
/// state the panel renders). That makes the number the one a click actually
/// costs.
#[cfg(debug_assertions)]
#[tauri::command]
pub fn Benchmark<R:Runtime>(App:AppHandle<R>, Count:u32) -> Result<String, String> {
	let Labels = ["Monitor26344", "Monitor22763"];

	let Count = Count.max(1);

	let mut Samples:Vec<u128> = Vec::with_capacity(Count as usize);

	let Start = std::time::Instant::now();

	for Step in 0..Count {
		let At = std::time::Instant::now();

		let mut Preferences:Settings = Load::Fn(&App)?;

		let Label = Labels[Step as usize % Labels.len()];

		let Word = match Step % 2 {
			0 => "light",
			_ => "dark",
		};

		PerSet::Fn(&mut Preferences.Ink, Some(Label), ModeOf::Fn(Word));

		let _ = Commit::Fn(&App, &Preferences)?;

		Samples.push(At.elapsed().as_micros());
	}

	let Total = Start.elapsed();

	Samples.sort_unstable();

	let At = |Fraction:f64| Samples[((Samples.len() - 1) as f64 * Fraction) as usize];

	let Mean = Samples.iter().sum::<u128>() / Samples.len() as u128;

	Ok(format!(
		"{Count} cycles in {:.2}s - mean {Mean}us, p50 {}us, p95 {}us, max {}us",
		Total.as_secs_f64(),
		At(0.50),
		At(0.95),
		Samples[Samples.len() - 1]
	))
}

/// The release build's copy: registered, and answers without doing anything.
#[cfg(not(debug_assertions))]
#[tauri::command]
pub fn Benchmark<R:Runtime>(_App:AppHandle<R>, _Count:u32) -> Result<String, String> {
	Err("Benchmark is a debug-build feature".into())
}
