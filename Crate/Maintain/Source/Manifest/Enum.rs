//! How one manifest stores the version.

/// How one manifest stores the version.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Enum {
	/// `Cargo.toml` -> `[workspace.package].version`
	Cargo,
	/// A `package.json` -> top-level `version`, which has to be there
	JSON,
	/// `tauri.conf.json` -> optional `version`; absent means "inherit the crate"
	Tauri,
}
