# Releasing

One version, one file to edit, one tag. This is the order that keeps the app,
the bundles and the crate on the same number.

---

## 1. Bump the version

`Cargo.toml` → `[workspace.package].version` is the truth, and everything else
either inherits it or is written from it:

- **The crate** inherits it with `version.workspace = true`, and `Publish.yml`
  reads it back through `cargo metadata` - this is the number crates.io sees.
- **The app and its bundles** read `CARGO_PKG_VERSION`, because
  `Crate/Round/tauri.conf.json` carries no `version` field at all and Tauri
  falls back to the manifest when the field is absent. Putting a literal back is
  supported, but it is then a copy that must be kept in step - and `Maintain`
  refuses a tree where it disagrees.
- **The two npm manifests** cannot inherit anything from a Cargo manifest, so
  `package.json` and `Package/Web/package.json` are the only copies left.

`Maintain` (`Crate/Maintain`, a Rust CLI in this workspace) is the single writer
for all of it:

```sh
cargo install --path Crate/Maintain # once, gives you `Maintain`

Maintain version            # version on stdout, manifest table on stderr
Maintain version bump patch # move one field, write it everywhere
Maintain version set 0.0.14 # an explicit version
```

The bare form is also the guard: it exits non-zero when a manifest has drifted
from `Cargo.toml`, and it prints the version alone on stdout, so
`$(Maintain version)` is exactly the tag suffix.

---

## 2. Tag

`Maintain release` does the whole ceremony in one step - bump, commit, tag, push
the branch, push the tag, and with `--publish` dispatch the crates.io publish:

```sh
Maintain release 0.0.14 --publish --dry-run # print the plan, change nothing
Maintain release 0.0.14 --publish           # do it
```

It refuses a version that does not move forward, refuses a dirty tree unless
`--allow-dirty`, and re-reads every manifest after writing to prove the number
landed everywhere.

The manual equivalent, when you want each step under your own hand:

```sh
VERSION=$(Maintain version)
git add Cargo.toml package.json Package/Web/package.json Crate/Round/tauri.conf.json
git commit -m "chore(release): $VERSION"
git tag -a "Round/v$VERSION" -m "Round/v$VERSION"
git push Source Current
git push Source "Round/v$VERSION"
```

The tag is the release trigger, and only `Build.yml` listens to it.

- `Build.yml` creates the GitHub release exactly once, then bundles macOS
  (arm64 + x86_64), Linux and Windows from one shared web artifact, writes
  `SHA256SUMS-<target>.txt` per platform, attaches everything, and a `Finalize`
  job fails if any of the four targets is missing.
- `Publish.yml` also runs on the tag, but only its `Verify` job: the packaging
  guard and the library consumer probe. **A tag push never publishes to
  crates.io.**

---

## 3. Publish the crate (opt-in)

crates.io versions are immutable, so publishing is a separate, explicit act:

```sh
Maintain publish # gh workflow run Publish.yml --ref Current -f publish_crates=true
gh run watch
```

What the job does:

1. reads the version from the workspace manifest;
2. skips everything when that version is already on the sparse index
   (`https://index.crates.io/pr/ou/pround`);
3. exchanges GitHub's OIDC identity for a short-lived crates.io token
   (`rust-lang/crates-io-auth-action`) - there is no stored registry secret;
4. `cargo publish -p pround --no-verify`.

`--no-verify` is deliberate. The verification build compiles the `round` binary,
whose `tauri::generate_context!` reads `Package/Web/Target` - a directory
outside the package and therefore absent from the tarball. Library consumers
never expand that macro, and the `Verify` job's consumer probe is what proves
the published surface.

### Trusted publisher on crates.io

The crate's trusted publisher must name, exactly:

- repository `PlayForm/Round`
- workflow file `Publish.yml`
- environment `Release`

A mismatch fails at token exchange, before `cargo publish` runs.

---

## 4. How to test a publish

Four levels, cheapest first. Only the last one burns a version.

### a. Package without uploading

```sh
cargo publish -p pround --dry-run --no-verify
```

Packages the crate and stops before upload. Expected shape:

```
Packaged 79 files, 462.3KiB (282.6KiB compressed)
Uploading pround v0.0.9 (/.../Crate/Round)
warning: aborting upload due to dry run
```

This catches a missing `readme`, a broken `include` list, or a manifest field
crates.io would reject - the three things that otherwise surface mid-release.

### b. Exercise the workflow without publishing

```sh
gh workflow run Publish.yml --ref Current # publish_crates defaults to false
gh run watch
```

Runs the packaging guard and the consumer probe (`cargo new` →
`cargo add pround --path` → `cargo build`) against the current tree. Nothing is
uploaded. This is the check that the _pipeline_ works, including the Linux
system dependencies and the toolchain.

### c. Rehearse the real thing with a throwaway version

The only way to prove the OIDC exchange end to end without spending `0.0.9`:

```sh
# bump only the cargo version - crates.io reads this one
# Cargo.toml -> [workspace.package].version = "0.0.9-rc.1"
gh workflow run Publish.yml --ref Current -f publish_crates=true
# then, once it is live:
cargo yank --version 0.0.9-rc.1 pround
```

A pre-release sorts below `0.0.9`, so publishing the real version afterwards is
unaffected, and `cargo add pround` keeps resolving the stable release. Revert
the manifest afterwards.

### d. The real publish

```sh
gh workflow run Publish.yml --ref Current -f publish_crates=true
```

`0.0.9` is free today - the index holds only `0.0.1`, the name reservation. If
the dispatch is run twice, the second run reads the index, sees the version, and
exits without touching the registry.

Verify afterwards:

```sh
curl -sL https://index.crates.io/pr/ou/pround
cargo new --lib /tmp/pround-probe && cd /tmp/pround-probe
cargo add pround && cargo build
```

`cargo install pround` from the registry will _not_ work: the published tarball
has no webview build, so the binary target cannot embed one. Install the app
from a checkout (`pnpm run Build` then `cargo install --path Crate/Round`) or
from the release bundle.

---

## 5. Local macOS release

`Release.sh` is the local equivalent of the tag build: it signs with `Round.key`
(gitignored), verifies the bundle is not hardened-runtime signed, packs the
`.app` with `ditto`, and uploads the zip + dmg to an existing release.

```sh
./Release.sh
```

The dmg step drives Finder through AppleScript, so the terminal running the
script needs macOS Automation permission for Finder. Without it the build dies
with `Finder got an error: Can't set toolbar visible ... (-10006)` **after**
`Round.app` has been built and signed; `pnpm tauri build --bundles app` is the
headless-safe target, and a macOS CI runner can produce the dmg.

---

## 6. After the release

- `git tag --list "Round/*"` and `gh release list` agree on the newest version.
- `crates.io` shows the same version as the tag (or intentionally lags, when the
  publish dispatch was skipped).
- `CHANGELOG.md` has a section for the version that was just cut - the
  `Unreleased` block moves down, it does not get rewritten.
