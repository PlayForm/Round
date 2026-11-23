//! Build script for the `pround` package.
//!
//! `tauri::generate_context!` embeds the frontend into the `round` binary at
//! compile time, reading `frontendDist` from `tauri.conf.json`. That directory
//! is `Package/Web/Target`: the web package builds it (`pnpm run Build`) and it
//! lives outside this crate on purpose - the crate stays a Rust crate, the web
//! package stays a web package, and Tauri packages from the outside directory.
//!
//! Only the BINARY needs those files. A library consumer (`cargo add pround`)
//! never expands `generate_context!`, so a missing frontend must not fail this
//! package's build - it is reported as a warning here and surfaces as one clear
//! error at the binary's own compile step.

// House style: constants are PascalCase; the lint that wants SCREAMING_SNAKE is
// switched off rather than the name.
#![allow(non_upper_case_globals)]

use std::path::{Path, PathBuf};

/// Where the web package writes its build, relative to this crate.
const Frontend:&str = "../../Package/Web/Target";

fn main() {
	let manifest =
		PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR is always set by cargo"));

	let frontend = std::env::var_os("ROUND_FRONTEND_DIST")
		.map(PathBuf::from)
		.unwrap_or_else(|| manifest.join(Frontend));

	println!("cargo:rerun-if-changed={}", frontend.display());

	println!("cargo:rerun-if-env-changed=ROUND_FRONTEND_DIST");

	if !Path::new(&frontend).join("index.html").exists() {
		println!(
			"cargo:warning=no frontend at {} - run `pnpm run Build` from the repository root before building the \
			 `round` binary. The `pround` library target does not need it.",
			frontend.display()
		);
	}

	tauri_build::build()
}
