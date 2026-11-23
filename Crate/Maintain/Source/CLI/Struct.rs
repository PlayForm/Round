//! The command surface, as clap parses it.

use clap::Parser;

use crate::CLI::Enum::Enum as Command;

#[derive(Parser)]
#[command(name = "Maintain", about = "Round's release CLI", version, disable_help_subcommand = true)]
pub struct Struct {
	#[command(subcommand)]
	pub Command:Command,
}
