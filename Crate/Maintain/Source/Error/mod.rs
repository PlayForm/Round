//! Everything that can go wrong, with the path that caused it.

use std::path::PathBuf;

pub type Result<T> = std::result::Result<T, Error>;

/// One variant per failure class, so a message always names the file or the
/// command instead of a bare "failed".
#[derive(Debug, thiserror::Error)]
pub enum Error {
	#[error("{path} could not be read: {source}")]
	Read { path:PathBuf, source:std::io::Error },

	#[error("{path} could not be written: {source}")]
	Write { path:PathBuf, source:std::io::Error },

	#[error("{path} is not valid {format}: {message}")]
	Parse { path:PathBuf, format:&'static str, message:String },

	#[error("{path} has no {what}")]
	Missing { path:PathBuf, what:&'static str },

	#[error("{0}")]
	Command(String),

	#[error("{0}")]
	Usage(String),

	#[error("{0}")]
	Release(String),
}
