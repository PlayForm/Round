//! Where the version lives, and the one writer for it.
//!
//! `Cargo.toml`'s `[workspace.package].version` is the truth: crates.io and
//! `Publish.yml` read it through `cargo metadata`, the crate inherits it with
//! `version.workspace = true`, and the Tauri bundle follows it because
//! `tauri.conf.json` carries no `version` at all (tauri-codegen falls back to
//! `CARGO_PKG_VERSION`). The two npm manifests are the only copies left.

use std::{
	fs,
	path::{Path, PathBuf},
};

use crate::{
	Error::{Error, Result},
	Version::Version,
};

/// How one manifest stores the version.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Kind {
	/// `Cargo.toml` -> `[workspace.package].version`
	Cargo,
	/// A `package.json` -> top-level `version`
	Json,
	/// `tauri.conf.json` -> optional `version`; absent means "inherit the crate"
	Tauri,
}

/// One file that carries the version.
pub struct Manifest {
	pub kind:Kind,
	pub path:PathBuf,
}

/// Every manifest, in the order a human reads them.
pub fn entries(root:&Path) -> Vec<Manifest> {
	let entry = |kind, relative:&str| Manifest { kind, path:root.join(relative) };

	vec![
		entry(Kind::Cargo, "Cargo.toml"),
		entry(Kind::Json, "package.json"),
		entry(Kind::Json, "Package/Web/package.json"),
		entry(Kind::Tauri, "Crate/Round/tauri.conf.json"),
	]
}

impl Manifest {
	/// The version this manifest holds. `None` only for a Tauri config that
	/// deliberately carries no version.
	pub fn read(&self) -> Result<Option<Version>> {
		let text = fs::read_to_string(&self.path).map_err(|source| Error::Read { path:self.path.clone(), source })?;

		match self.kind {
			Kind::Cargo => cargo_read(&self.path, &text).map(Some),
			Kind::Json => json_read(&self.path, &text, true),
			Kind::Tauri => json_read(&self.path, &text, false),
		}
	}

	pub fn write(&self, version:&Version) -> Result<()> {
		let text = fs::read_to_string(&self.path).map_err(|source| Error::Read { path:self.path.clone(), source })?;

		let written = match self.kind {
			Kind::Cargo => cargo_write(&self.path, &text, version)?,
			Kind::Json | Kind::Tauri => {
				let current = json_read(&self.path, &text, self.kind == Kind::Json)?;

				// Absent is the intended state for the Tauri config, not a gap.
				current.map_or(text.clone(), |current| {
					replace_json_version(&text, &current.to_string(), &version.to_string())
						.unwrap_or_else(|| text.clone())
				})
			}
		};

		fs::write(&self.path, written).map_err(|source| Error::Write { path:self.path.clone(), source })
	}
}

/// The version every manifest agrees on, taken from `Cargo.toml`.
pub fn current(root:&Path) -> Result<Version> {
	let cargo = entries(root).into_iter().find(|manifest| manifest.kind == Kind::Cargo);

	cargo
		.ok_or_else(|| Error::Missing { path:root.join("Cargo.toml"), what:"[workspace.package].version" })?
		.read()?
		.ok_or_else(|| Error::Missing { path:root.join("Cargo.toml"), what:"[workspace.package].version" })
}

/// Every manifest whose version differs from `version`.
pub fn drift(root:&Path, version:&Version) -> Result<Vec<PathBuf>> {
	entries(root).into_iter().try_fold(Vec::new(), |mut stale, manifest| {
		let held = manifest.read()?;

		let _ = held.filter(|held| held != version).map(|_| stale.push(manifest.path));

		Ok(stale)
	})
}

// --- Cargo -------------------------------------------------------------------

fn cargo_read(path:&Path, text:&str) -> Result<Version> {
	let document = text.parse::<toml_edit::DocumentMut>().map_err(|error| Error::Parse {
		path:path.to_path_buf(),
		format:"TOML",
		message:error.to_string(),
	})?;

	let field = document
		.get("workspace")
		.and_then(|workspace| workspace.get("package"))
		.and_then(|package| package.get("version"))
		.and_then(|version| version.as_str())
		.ok_or_else(|| Error::Missing { path:path.to_path_buf(), what:"[workspace.package].version" })?;

	field.parse()
}

fn cargo_write(path:&Path, text:&str, version:&Version) -> Result<String> {
	let mut document = text.parse::<toml_edit::DocumentMut>().map_err(|error| Error::Parse {
		path:path.to_path_buf(),
		format:"TOML",
		message:error.to_string(),
	})?;

	document["workspace"]["package"]["version"] = toml_edit::value(version.to_string());

	Ok(document.to_string())
}

// --- JSON --------------------------------------------------------------------

fn json_read(path:&Path, text:&str, required:bool) -> Result<Option<Version>> {
	let parsed:serde_json::Value = serde_json::from_str(text).map_err(|error| Error::Parse {
		path:path.to_path_buf(),
		format:"JSON",
		message:error.to_string(),
	})?;

	match parsed.get("version") {
		Some(serde_json::Value::String(field)) => field.parse().map(Some),
		None if !required => Ok(None),
		_ => Err(Error::Missing { path:path.to_path_buf(), what:"string `version`" }),
	}
}

/// Rewrite the first `"version": "..."` pair, but only when the value found there
/// is the one JSON parsing reported. Without that check a nested `version`
/// elsewhere in the file would be silently edited instead.
fn replace_json_version(text:&str, expected:&str, version:&str) -> Option<String> {
	let key = "\"version\"";
	let key_at = text.find(key)?;
	let after_key = &text[key_at + key.len()..];
	let colon_at = after_key.find(':')?;
	let after_colon = &after_key[colon_at + 1..];
	let open_at = after_colon.find('"')?;
	let value_at = open_at + 1;
	let close_at = value_at + after_colon[value_at..].find('"')?;
	let found = &after_colon[value_at..close_at];

	(found == expected).then(|| {
		let start = key_at + key.len() + colon_at + 1 + value_at;

		format!("{}{}{}", &text[..start], version, &text[start + found.len()..])
	})
}
