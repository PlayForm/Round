//! The `Maintain` binary.

// House style: locals and parameters are PascalCase, the same as the library's.
#![allow(non_snake_case)]

use Maintain::CLI;

fn main() {
	match CLI::Run::Fn() {
		Ok(()) => {}
		Err(Error) => {
			// `{:#}` is the whole chain on one line: every context names its
			// operation, and the last link is the tool or the file that failed.
			eprintln!("Error: {Error:#}");

			std::process::exit(1);
		}
	}
}
