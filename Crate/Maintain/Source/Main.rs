//! The `maintain` binary.

use round_maintain::CLI;

fn main() {
	match CLI::run() {
		Ok(()) => {}
		Err(error) => {
			eprintln!("Error: {error}");

			std::process::exit(1);
		}
	}
}
