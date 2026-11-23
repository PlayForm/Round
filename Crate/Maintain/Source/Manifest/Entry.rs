//! Every manifest, in the order a human reads them.

use std::path::Path;

use crate::Manifest::{Enum::Enum as Kind, Struct::Struct};

pub fn Fn(Root:&Path) -> Vec<Struct> {
	let Entry = |Kind:Kind, Relative:&str| Struct { Kind, Path:Root.join(Relative) };

	vec![
		Entry(Kind::Cargo, "Cargo.toml"),
		Entry(Kind::JSON, "package.json"),
		Entry(Kind::JSON, "Package/Web/package.json"),
		Entry(Kind::Tauri, "Crate/Round/tauri.conf.json"),
	]
}
