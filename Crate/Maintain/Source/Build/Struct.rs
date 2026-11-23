//! What a build was asked for.

/// The two knobs `Maintain build` exposes, as the CLI parsed them.
pub struct Struct {
	/// `--debug` -> `pnpm tauri build --debug`
	pub Debug:bool,
	/// `--bundles <list>` -> `pnpm tauri build --bundles <list>`
	pub Bundles:Option<String>,
}
