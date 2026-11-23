//! The empty section a release leaves above the one it just cut.

/// A release renames `## Unreleased` to the version it is releasing and leaves
/// a fresh, empty `Unreleased` above it. Rewriting the block below is the one
/// thing the ceremony must not do: what was written under `Unreleased` is the
/// record of the release, and it moves down intact.
pub fn Fn() -> [String; 6] {
	[
		crate::Changelog::Unreleased::Const.to_owned(),
		String::new(),
		"_(nothing yet)_".to_owned(),
		String::new(),
		"---".to_owned(),
		String::new(),
	]
}
