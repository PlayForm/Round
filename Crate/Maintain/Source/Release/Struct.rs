//! What a release was asked for.

use crate::Version::Struct::Struct as Number;

pub struct Struct {
	/// The version being cut, which has to sort after the current one.
	pub Version:Number,
	/// `--publish` -> dispatch `Publish.yml` with `publish_crates=true` last.
	pub Publish:bool,
	/// `--dry-run` -> print the plan and change nothing.
	pub DryRun:bool,
	/// `--allow-dirty` -> release a tree that has uncommitted work in it.
	pub AllowDirty:bool,
}
