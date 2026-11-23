//! Cutting a release: bump, commit, tag, push, and optionally publish.

use std::path::{Path, PathBuf};

use crate::{
	Error::{Error, Result},
	Manifest,
	Process,
	Version::Version,
};

pub struct Options {
	pub version:Version,
	pub publish:bool,
	pub dry_run:bool,
	pub allow_dirty:bool,
}

/// One command in the ceremony, kept as data so `--dry-run` prints exactly what
/// a real run would execute instead of a prose description of it.
struct Step {
	program:&'static str,
	args:Vec<String>,
}

impl Step {
	fn new(program:&'static str, args:&[&str]) -> Self {
		Self { program, args:args.iter().map(|arg| (*arg).to_owned()).collect() }
	}

	fn render(&self) -> String { format!("$ {} {}", self.program, self.args.join(" ")) }

	fn run(&self, root:&Path) -> Result<()> {
		let args:Vec<&str> = self.args.iter().map(String::as_str).collect();

		Process::run(root, self.program, &args)
	}
}

pub fn run(root:&Path, options:&Options) -> Result<()> {
	let current = Manifest::current(root)?;

	// Tags and crates.io versions are append-only: a release that does not move
	// forward can only fail later, after the tag exists.
	options
		.version
		.is_after(&current)
		.then_some(())
		.ok_or_else(|| Error::Release(format!("{} is not after the current {current}", options.version)))?;

	options.allow_dirty.then_some(()).map_or_else(|| clean(root), Ok)?;

	let branch = Process::capture(root, "git", &["rev-parse", "--abbrev-ref", "HEAD"])?;
	let remote = remote(root)?;
	let tag = options.version.tag();

	let mut add:Vec<String> = vec!["add".to_owned()];
	Manifest::entries(root).iter().for_each(|manifest| {
		add.push(manifest.path.strip_prefix(root).unwrap_or(&manifest.path).display().to_string());
	});

	let mut steps = vec![
		Step { program:"git", args:add },
		Step::new("git", &["commit", "-m", &format!("chore(release): {}", options.version)]),
		Step::new("git", &["tag", "-a", &tag, "-m", &tag]),
		Step::new("git", &["push", &remote, &branch]),
		Step::new("git", &["push", &remote, &tag]),
	];

	steps.extend(options.publish.then(|| {
		Step::new(
			"gh",
			&["workflow", "run", "Publish.yml", "--ref", &branch, "-f", "publish_crates=true"],
		)
	}));

	if options.dry_run {
		println!("would write {} to every manifest, then run:", options.version);
		steps.iter().for_each(|step| println!("{}", step.render()));

		return Ok(());
	}

	Manifest::entries(root)
		.iter()
		.try_for_each(|manifest| manifest.write(&options.version))?;

	let stale = Manifest::drift(root, &options.version)?;

	stale
		.is_empty()
		.then_some(())
		.ok_or_else(|| Error::Release(format!("{} did not take everywhere: {}", options.version, join(&stale))))?;

	println!("{} written to every manifest.", options.version);

	steps.iter().try_for_each(|step| step.run(root))
}

/// Refuse to release a tree that has work in it, so the tag always names a
/// commit whose content is exactly what was verified.
fn clean(root:&Path) -> Result<()> {
	let dirty = Process::capture(root, "git", &["status", "--porcelain"])?;

	dirty.is_empty().then_some(()).ok_or_else(|| {
		Error::Release(format!(
			"{} file(s) uncommitted; commit them or pass --allow-dirty",
			dirty.lines().count()
		))
	})
}

/// The remote the release pushes to. This repository names it `Source`; a
/// clone that only has `origin` still works.
fn remote(root:&Path) -> Result<String> {
	let listed = Process::capture(root, "git", &["remote"])?;

	let names:Vec<&str> = listed.lines().map(str::trim).filter(|name| !name.is_empty()).collect();

	names
		.iter()
		.find(|name| **name == "Source")
		.or_else(|| names.first())
		.map(|name| (*name).to_owned())
		.ok_or_else(|| Error::Release("this repository has no git remote".to_owned()))
}

fn join(paths:&[PathBuf]) -> String {
	paths
		.iter()
		.map(|path| path.display().to_string())
		.collect::<Vec<_>>()
		.join(", ")
}
