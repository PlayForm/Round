# Unreleased

- Repository split into `Crate/Round` (the Rust crate) and `Package/Web` (the
  webview package); the crate is published to crates.io as **`pround`**, with
  the library usable through `cargo add pround` and the `round` binary embedded
  from `Package/Web/Target` at compile time
- Rust engine split into typed PascalCase modules (`Action`, `Effect`,
  `Message`, `Mode`, `Overlay`, `Screen`, `Settings`, `Tray`); the tray menu is
  a match over a domain enum instead of string comparisons, and `regex` was
  dropped for an ASCII filter
- Workspace root with edition 2024, floating `nightly` toolchain,
  `rustfmt.toml`, one `.cargo/config.toml`, and build profiles moved out of the
  config file (cargo ignores `[profile]` there)
- PostCSS config rewritten as ESM: cssnano 9 and
  postcss-combine-duplicated-selectors 11 are ESM-only, so the old
  `postcss.config.cjs` failed the webview build outright
- Dependencies refreshed to the newest release inside each major (Solid 1.9,
  Vite 8, Tauri 2.12, tauri-plugin-store 2.5), `pnpm-lock.yaml` tracked,
  `Cargo.lock` untracked by policy
- Workflows replaced: `Check.yml` (webview build, fmt, clippy, check, packaging
  guard), `Build.yml` (tag matrix for macOS arm64/x64, Linux, Windows with
  checksums and a finalize gate), `Publish.yml` (crates.io after a successful
  tag build), a heartbeat-only `Auto.yml`; `Rust.yml`, `Node.yml` and `NPM.yml`
  removed
- README rewritten around the new layout, install paths, and CI

# 0.0.9

- macOS native: overlays sit at `NSStatusWindowLevel` with
  `canJoinAllSpaces | stationary | ignoresCycle | fullScreenAuxiliary`
  collection behaviour and `setIgnoresMouseEvents:YES`, covering the menu bar
  across Spaces and fullscreen apps
- Windows taskbar overlay: `SetWindowPos(HWND_TOPMOST, ..., SWP_NOACTIVATE)`
  after creation, plus ex-styles
  `WS_EX_TOOLWINDOW | WS_EX_NOACTIVATE | WS_EX_TRANSPARENT`;
  `skip_taskbar(true)` is now Windows-only
- New tray entry **No Dock ⚓** (macOS only) toggles
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

# 0.0.8

- Restoration

# 0.0.7

- Cleanup

# 0.0.6

- Cleanup

# 0.0.5

- Fixes scale when using multiple monitors

# 0.0.4

- Adds multi-monitor setup

# 0.0.3

- Cleanup

# 0.0.2

- Bug fix

# 0.0.1

- Initial commit
