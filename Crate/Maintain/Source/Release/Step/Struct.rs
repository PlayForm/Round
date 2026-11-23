//! One command in the ceremony.

/// One command in the ceremony, kept as data so `--dry-run` prints exactly what
/// a real run would execute instead of a prose description of it.
///
/// The fields are the crate's own business: a step is built by
/// [`New::Fn`](crate::Release::Step::New::Fn) and read by
/// [`Render::Fn`](crate::Release::Step::Render::Fn) and
/// [`Run::Fn`](crate::Release::Step::Run::Fn), never assembled from outside.
pub struct Struct {
	pub(crate) Program:&'static str,
	pub(crate) Args:Vec<String>,
}
