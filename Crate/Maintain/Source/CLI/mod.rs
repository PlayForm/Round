//! The command surface.

use std::path::Path;

use clap::{Parser, Subcommand};

use crate::{
	Build,
	Check,
	Error::{Error, Result},
	Manifest,
	Process,
	Release,
	Root,
	Version::{Part, Version},
};

#[derive(Parser)]
#[command(name = "maintain", about = "Round's release CLI", version, disable_help_subcommand = true)]
pub struct CLI {
	#[command(subcommand)]
	pub command:Command,
}

#[derive(Subcommand)]
pub enum Command {
	/// Show the version, and fail when a manifest disagrees with Cargo.toml.
	Version {
		#[command(subcommand)]
		action:Option<VersionCommand>,
	},
	/// Run the gates CI runs: fmt, clippy -D warnings, check.
	Check,
	/// Build the webview package and the Tauri bundle.
	Build {
		#[arg(long)]
		debug:bool,
		#[arg(long)]
		bundles:Option<String>,
	},
	/// Bump, commit, tag and push a release.
	Release {
		version:Version,
		#[arg(long)]
		publish:bool,
		#[arg(long)]
		dry_run:bool,
		#[arg(long)]
		allow_dirty:bool,
	},
	/// Dispatch Publish.yml with publish_crates=true.
	Publish {
		#[arg(long)]
		dry_run:bool,
	},
}

#[derive(Subcommand)]
pub enum VersionCommand {
	/// Write this version to every manifest.
	Set { version:Version },
	/// Move one field and write the result everywhere.
	Bump { part:Part },
	/// Re-propagate `Cargo.toml`'s version after editing it by hand.
	Sync,
}

pub fn run() -> Result<()> {
	let cli = CLI::parse();

	let root = Root::find()?;

	match cli.command {
		Command::Version { action } => version(&root, action),
		Command::Check => Check::run(&root),
		Command::Build { debug, bundles } => Build::run(&root, &Build::Options { debug, bundles }),
		Command::Release { version, publish, dry_run, allow_dirty } => {
			Release::run(&root, &Release::Options { version, publish, dry_run, allow_dirty })
		}
		Command::Publish { dry_run } => publish(&root, dry_run),
	}
}

fn version(root:&Path, action:Option<VersionCommand>) -> Result<()> {
	match action {
		Some(VersionCommand::Set { version }) => write(root, &version),
		Some(VersionCommand::Bump { part }) => write(root, &Manifest::current(root)?.bump(part)),
		Some(VersionCommand::Sync) => write(root, &Manifest::current(root)?),
		None => report(root),
	}
}

fn write(root:&Path, version:&Version) -> Result<()> {
	Manifest::entries(root)
		.iter()
		.try_for_each(|manifest| manifest.write(version))?;

	report(root)
}

/// The table is for a human and goes to stderr; stdout carries the version
/// alone, so `$(maintain version)` is the version.
fn report(root:&Path) -> Result<()> {
	let current = Manifest::current(root)?;

	let rows = Manifest::entries(root).into_iter().try_fold(Vec::new(), |mut rows, manifest| {
		let held = manifest.read()?;

		rows.push((manifest.path, held));

		Ok::<_, Error>(rows)
	})?;

	let shown:Vec<(String, String)> = rows
		.iter()
		.map(|(path, held)| {
			let file = path.strip_prefix(root).unwrap_or(path).display().to_string();

			let value = held
				.as_ref()
				.map(ToString::to_string)
				.unwrap_or_else(|| format!("inherits {current}"));

			(file, value)
		})
		.collect();

	let width = shown.iter().map(|(file, _)| file.len()).max().unwrap_or(0);

	shown.iter().for_each(|(file, value)| eprintln!("{file:width$}  {value}"));

	let stale:Vec<String> = rows
		.iter()
		.filter_map(|(path, held)| {
			held.as_ref()
				.filter(|held| **held != current)
				.map(|_| path.strip_prefix(root).unwrap_or(path).display().to_string())
		})
		.collect();

	stale.is_empty().then(|| println!("{current}")).ok_or_else(|| {
		Error::Usage(format!(
			"{} manifest(s) disagree with Cargo.toml ({current}): run `maintain version --sync`",
			stale.len()
		))
	})
}

fn publish(root:&Path, dry_run:bool) -> Result<()> {
	let branch = Process::capture(root, "git", &["rev-parse", "--abbrev-ref", "HEAD"])?;

	let args = [
		"workflow",
		"run",
		"Publish.yml",
		"--ref",
		branch.as_str(),
		"-f",
		"publish_crates=true",
	];

	dry_run
		.then(|| println!("$ gh {}", args.join(" ")))
		.map_or_else(|| Process::run(root, "gh", &args), |()| Ok(()))
}
