//! What a release run does with its plan.

/// `--dry-run` is a `bool` because that is what a flag is; the decision it
/// stands for is which of the two whole behaviours a release runs.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Enum {
	/// Print the plan and change nothing.
	Preview,
	/// Promote the changelog, write the manifests and execute the plan.
	Perform,
}
