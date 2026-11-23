# Changelog

Every notable change to Round, newest first. Versions are the application's
(`Cargo.toml` → `[workspace.package].version`, which the app, the bundles and
the crate on crates.io all follow), so a release is one version across the app,
the npm packages and `pround`.

---

## Unreleased

### Repository

- One source of truth for the version: `Cargo.toml` →
  `[workspace.package].version`. `Crate/Round/tauri.conf.json` no longer carries
  a `version` field, so the app and its bundles take `CARGO_PKG_VERSION` from the
  workspace manifest, and `Scripts/Version.mjs` writes the two npm manifests that
  cannot inherit it - `node Scripts/Version.mjs 0.0.11` replaces the four-place
  edit. `Release.sh` reads the same source and refuses a drifted tree instead of
  offering a tag that disagrees with the crate.

---

## 0.0.10

### Repository

- Split into **`Crate/Round`** (the Rust crate) and **`Package/Web`** (the
  webview package). The crate is published to crates.io as **`pround`**: the
  library is `cargo add pround`, the `round` binary embeds `Package/Web/Target`
  at compile time and is what the bundles ship.
- One pnpm workspace at the root with `Package/Web` as its only member; the root
  `package.json` delegates with `pnpm --filter @application/round-web`, so
  `pnpm run Build` and `pnpm tauri build` work from the root regardless of the
  caller's directory.
- `@tauri-apps/cli` moved to the root devDependencies: the Tauri CLI finds
  `Crate/Round/tauri.conf.json` by walking down from the root, and treats the
  root as the frontend directory.
- Architecture graphs moved to `Documentation/` (`Knowledge.dot`,
  `Knowledge.svg`) and rewritten for the new module map; the generated Tauri ACL
  schemas under `Crate/Round/gen/` are no longer tracked (tauri-build rewrites
  them on every build).

### Crate (`pround`)

- Split into typed PascalCase modules: `Action`, `Effect`, `Message`, `Mode`,
  `Overlay`, `Screen`, `Settings`, `Tray`, behind a `Source/Library.rs` facade
  that only declares them.
- The tray menu is one match over `Action` (which owns both the menu identifier
  and the menu text) returning an `Effect` (`Persist`, `Dock(hidden)`, `Exit`),
  instead of string comparisons re-derived at the call site.
- `Mode` is an enum that serialises to the lowercase word the webview reads;
  unknown store values fall back to the default instead of reaching CSS as a
  typo. `Settings::load` falls back per field, so a hand-edited store costs one
  setting, never the launch.
- `regex` dropped: monitor labels are sanitised with an ASCII filter, which is
  the same rule the regex encoded.
- Workspace root with edition 2024, `rust-version = "1.90"` (the floor Tauri
  2.12 imposes - a lower value makes cargo's MSRV-aware resolver lock older
  transitive crates), a floating `nightly` toolchain, `rustfmt.toml`, one
  `.cargo/config.toml`, and build profiles moved into the manifest (cargo
  ignores `[profile]` in a config file).
- `build.rs` reports a missing webview build as a `cargo:warning` instead of
  failing the package: a library consumer never expands `generate_context!` and
  must not be blocked by a frontend it does not need.
- The crate name was reserved on crates.io as `pround` 0.0.1 (a one-version
  stub); this release is 0.0.10, which supersedes it.

### Webview (`@application/round-web`)

- `postcss.config.cjs` replaced by `postcss.config.mjs`: cssnano 9 and
  postcss-combine-duplicated-selectors 11 are ESM-only, so `require(...)`
  returned a module namespace instead of a plugin and the webview build failed
  outright.
- `cssnano-preset-advanced` dropped from the manifest - cssnano 9 carries the
  advanced preset itself.
- `window.settings` is declared once, in `Source/Type/Settings.ts`, together
  with the two event payload types (`SizeMessage`, `ModeMessage`), instead of
  being redeclared in every module that touches it.
- The dev server binds `127.0.0.1` and `devUrl` names the same literal:
  `localhost` resolves to `::1` first on macOS, so the default host refused
  every IPv4 dial while Vite printed "ready".

### Dependencies

- npm: `solid-js` 1.9.20, `vite` 8.3.8, `vite-plugin-solid` 2.11.17,
  `@tauri-apps/api` 2.12.5, `@tauri-apps/cli` 2.12.6, `postcss` 8.5.33,
  `cssnano` 9.9.0, `autoprefixer` 10.7.0, `@types/node` 26.6.6. Tailwind stays
  on 3.4.19 (4.x replaces the config and plugin chain this package is built on).
- cargo: `tauri` 2.12, `tauri-build` 2.7, `tauri-plugin-store` 2.5, `objc2` 0.6,
  `windows-sys` 0.61, `serde`/`serde_json` 1.x.
- `pnpm-lock.yaml` is tracked; `Cargo.lock` is not (the crate publishes to
  crates.io and every consumer resolves its own graph).

### CI

- `Check.yml`: builds the webview once and shares it as an artifact, then runs
  `cargo fmt --check`, `cargo clippy -D warnings`, `cargo check`, and a
  packaging guard that asserts the crate's own files are in the published
  tarball.
- `Build.yml`: on a `Round/v*` tag, creates the release once, bundles macOS
  (arm64 and x86_64), Linux and Windows from the shared web artifact, stages the
  bundles, writes per-target `SHA256SUMS`, and fails in a `Finalize` job when
  the matrix is incomplete.
- `Publish.yml`: runs the packaging guard and a library consumer probe on every
  tag; publishing to crates.io is an explicit `workflow_dispatch` with
  `publish_crates=true`, authenticated through crates.io trusted publishing
  (OIDC, no stored registry token) and skipped when the version is already live.
- `Auto.yml` now commits only `.github/Update.md` with a real message instead of
  sweeping the working tree with `git add .`.
- `Rust.yml`, `Node.yml` and `NPM.yml` removed; Dependabot covers GitHub
  Actions, both npm manifests and `Crate/Round` (Tailwind 4 pinned off).

### Documentation

- README rewritten around the split, the install paths and the workflows.
- `Documentation/Architecture.md` (module map, event flow, webview contract) and
  `Documentation/Releasing.md` (version, tag, publish, and how to test a
  publish) added; `Documentation/Index.md` maps the set.
- `Release.sh` updated for the new bundle paths and `pnpm tauri build`, with the
  Finder-automation requirement for the dmg step recorded.

---

## 0.0.9

- macOS native: overlays sit at `NSStatusWindowLevel` with
  `canJoinAllSpaces | stationary | ignoresCycle | fullScreenAuxiliary`
  collection behaviour and `setIgnoresMouseEvents:YES`, covering the menu bar
  across Spaces and fullscreen apps
- Windows taskbar overlay: `SetWindowPos(HWND_TOPMOST, ..., SWP_NOACTIVATE)`
  after creation, plus ex-styles
  `WS_EX_TOOLWINDOW | WS_EX_NOACTIVATE | WS_EX_TRANSPARENT`;
  `skip_taskbar(true)` is now Windows-only
- New tray entry **No Dock ⚓** (macOS only) toggles
  `NSApplicationActivationPolicy.Accessory`; persisted, checked when active
- Migrated to Tauri 2 (`tray-icon`, `macos-private-api`); replaced legacy
  `SystemTray` with `TrayIconBuilder` + `WebviewWindowBuilder`; ported
  `tauri-plugin-store` to v2 (`StoreExt::store`)
- Frontend ported to Tauri 2 API (`getCurrentWebviewWindow`), typed event
  payloads, removed dead `appWindow` import; CSS module declaration added
- Dedicated tray template icon (`icons/tray.png`) instead of using the bundle
  icon (which rendered as a black square under `iconAsTemplate`); full app icon
  set regenerated from `Logo.svg` with a squircle alpha mask
- Renamed product from "Round Windows" to "Round" everywhere (`productName`,
  npm, crate, knowledge graphs)
- README rewritten and tightened; em quad (U+2001) used between text and emoji
  throughout the project for visual consistency
- Linux builds compile but are untested

## 0.0.8

- Restoration

## 0.0.7

- Cleanup

## 0.0.6

- Cleanup

## 0.0.5

- Fixes scale when using multiple monitors

## 0.0.4

- Adds multi-monitor setup

## 0.0.3

- Cleanup

## 0.0.2

- Bug fix

## 0.0.1

- Initial commit
