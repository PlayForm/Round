#!/usr/bin/env node
//
// One version, one file to edit.
//
// `Cargo.toml` -> `[workspace.package].version` is the truth. Three things
// already read it without help:
//
//   - crates.io / `Publish.yml`, through `cargo metadata`;
//   - the `pround` crate, through `version.workspace = true`;
//   - the app and its bundles, because `Crate/Round/tauri.conf.json` carries no
//     `version` at all and Tauri then falls back to `CARGO_PKG_VERSION`.
//
// The two npm manifests cannot inherit anything from a Cargo manifest, so they
// are the only copies left - and this script is what keeps them equal.
//
// Usage:
//   node Scripts/Version.mjs            the version on stdout, the manifest
//                                       table on stderr; exit 1 on drift
//   node Scripts/Version.mjs 0.0.11     set the version everywhere
//   node Scripts/Version.mjs --sync     re-propagate after editing Cargo.toml
//
// Exit codes: 0 in step, 1 drift or write failure, 2 usage error.
import { readFileSync, writeFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";

const ROOT = dirname(dirname(fileURLToPath(import.meta.url)));

const TRUTH = "Cargo.toml";
const CARGO_SECTION = "[workspace.package]";
const MANIFESTS = ["package.json", "Package/Web/package.json"];
// Only checked, never required: the bundle version comes from Cargo.toml while
// this field is absent. If somebody puts it back, it must not drift silently.
const TAURI = "Crate/Round/tauri.conf.json";

const SEMVER = /^\d+\.\d+\.\d+(?:-[0-9A-Za-z.-]+)?(?:\+[0-9A-Za-z.-]+)?$/;

const path = (relative) => join(ROOT, relative);
const read = (relative) => readFileSync(path(relative), "utf8");

function fail(message) {
	console.error(`Error: ${message}`);
	process.exit(1);
}

// --- Cargo.toml --------------------------------------------------------------
// Line-based on purpose: a TOML parser would rewrite the whole manifest, and
// this file is read by humans and by `cargo publish`.

// A CRLF checkout must not come back with one LF-terminated line, so the file's
// own terminator is what the rewrite rejoins with.
function splitLines(text) {
	const eol = text.includes("\r\n") ? "\r\n" : "\n";

	return { eol, lines: text.split(eol) };
}

function cargoVersion() {
	const { lines } = splitLines(read(TRUTH));
	let inside = false;

	for (let index = 0; index < lines.length; index += 1) {
		const line = lines[index].trim();

		if (line.startsWith("[")) {
			inside = line === CARGO_SECTION;
			continue;
		}

		if (!inside) continue;

		const found = /^version\s*=\s*"([^"]+)"/.exec(line);
		if (found) return { version: found[1], index };
	}

	return fail(`${TRUTH} has no \`version\` in \`${CARGO_SECTION}\``);
}

function setCargoVersion(version) {
	const { eol, lines } = splitLines(read(TRUTH));
	const { index } = cargoVersion();
	const indent = /^\s*/.exec(lines[index])[0];

	lines[index] = `${indent}version = "${version}"`;
	writeFileSync(path(TRUTH), lines.join(eol));
}

// --- JSON manifests ----------------------------------------------------------
// Parsed to read, replaced textually to write: re-serialising would reformat
// every unrelated line of a hand-maintained manifest.

function jsonVersion(relative) {
	const text = read(relative);

	let parsed;
	try {
		parsed = JSON.parse(text);
	} catch (error) {
		return fail(`${relative} is not valid JSON (${error.message})`);
	}

	if (typeof parsed.version !== "string") {
		return fail(`${relative} has no string \`version\` field`);
	}

	return parsed.version;
}

// Matches the first `"version": "..."` pair. Group 2 is the value, so a caller
// can prove the match it is about to rewrite is the manifest's own field and not
// a nested `version` further down.
const VERSION_FIELD = /("version"\s*:\s*")([^"]*)(")/;

function setJsonVersion(relative, version) {
	const text = read(relative);
	const found = VERSION_FIELD.exec(text);

	if (!found) return fail(`${relative} has no \`version\` field to rewrite`);

	// The value the regex found must be the one JSON parsing reports at the top
	// level; otherwise the rewrite would land on somebody else's `version`.
	if (found[2] !== jsonVersion(relative)) {
		return fail(
			`${relative} has a nested \`version\` this script cannot locate`,
		);
	}

	writeFileSync(
		path(relative),
		text.replace(VERSION_FIELD, `$1${version}$3`),
	);

	const seen = jsonVersion(relative);
	if (seen !== version)
		return fail(`${relative} still reads ${seen} after writing`);
}

function tauriVersion() {
	const text = read(TAURI);

	let parsed;
	try {
		parsed = JSON.parse(text);
	} catch (error) {
		return fail(`${TAURI} is not valid JSON (${error.message})`);
	}

	return typeof parsed.version === "string" ? parsed.version : null;
}

function setTauriVersion(version) {
	const current = tauriVersion();

	// Absent is the intended state, not a gap: Tauri then reads CARGO_PKG_VERSION.
	if (current === null) return;

	const text = read(TAURI);
	const found = VERSION_FIELD.exec(text);

	// Only a top-level literal is ours to rewrite; anything else is refused
	// rather than silently edited in the wrong place.
	if (!found || found[2] !== current) {
		return fail(
			`${TAURI} has a nested \`version\` this script cannot locate`,
		);
	}

	writeFileSync(path(TAURI), text.replace(VERSION_FIELD, `$1${version}$3`));
}

// --- Reporting ---------------------------------------------------------------

function current() {
	const { version } = cargoVersion();
	const tauri = tauriVersion();

	return [
		{ file: TRUTH, version, checked: true },
		...MANIFESTS.map((file) => ({
			file,
			version: jsonVersion(file),
			checked: true,
		})),
		{
			file: TAURI,
			version: tauri ?? "inherits Cargo.toml",
			checked: tauri !== null,
		},
	];
}

function report(rows, stream) {
	const width = Math.max(...rows.map(({ file }) => file.length));

	for (const { file, version } of rows) {
		stream.write(`${file.padEnd(width)}  ${version}\n`);
	}
}

// An absent `version` in tauri.conf.json is the intended state, not a gap: the
// row is only compared when the field is actually present.
function drift(rows, truth) {
	return rows.filter(({ version, checked }) => checked && version !== truth);
}

function sync(version) {
	if (!SEMVER.test(version)) fail(`"${version}" is not a semver version`);

	setCargoVersion(version);
	MANIFESTS.forEach((file) => setJsonVersion(file, version));
	setTauriVersion(version);

	const rows = current();
	report(rows, process.stdout);

	if (drift(rows, version).length > 0) {
		return fail(`these manifests did not take ${version}`);
	}

	console.log(`\n${version} written to every manifest.`);
}

// --- Entry point -------------------------------------------------------------

const [argument] = process.argv.slice(2);

if (argument === undefined) {
	const rows = current();
	const truth = rows[0].version;

	// The table is for a human, so it goes to stderr; stdout carries the version
	// alone, which is what `$(node Scripts/Version.mjs)` must capture.
	report(rows, process.stderr);

	const stale = drift(rows, truth);
	if (stale.length > 0) {
		fail(
			`${stale.length} manifest(s) disagree with ${TRUTH} (${truth}). ` +
				"Run `node Scripts/Version.mjs --sync`.",
		);
	}

	console.log(truth);
} else if (argument === "--sync") {
	sync(cargoVersion().version);
} else if (argument.startsWith("-")) {
	console.error("Usage: node Scripts/Version.mjs [<version> | --sync]");
	process.exit(2);
} else {
	sync(argument);
}
