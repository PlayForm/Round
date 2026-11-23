//! The `pnpm tauri build` arguments a set of options asks for.

use crate::Build::Struct::Struct as Build;

/// Built as a list rather than appended branch by branch, so the flag order is
/// one reading of this function and not a consequence of which flags were set.
pub fn Fn(Option:&Build) -> Vec<String> {
	let mut Args:Vec<String> = ["tauri", "build"].map(str::to_owned).to_vec();

	Args.extend(Option.Debug.then_some("--debug").map(str::to_owned));

	Args.extend(
		Option
			.Bundles
			.iter()
			.flat_map(|Bundles| ["--bundles", Bundles.as_str()])
			.map(str::to_owned),
	);

	Args
}
