# Changelog

Every notable change to Round, newest first. Versions are the application's
(`Cargo.toml` → `[workspace.package].version`, which the app, the bundles and
the crate on crates.io all follow), so a release is one version across the app,
the npm packages and `pround`.

---

## 0.0.14

### Fixed

- **The panel's `↺` reset buttons did nothing.** `useContext` resolves through the
  reactive owner, and there is no owner inside a click handler - calling it there
  returned the context's empty default, so the button ran `undefined.call`. The
  context is now read once during the component's setup and captured, which is the
  only place it resolves.

---

## 0.0.13

### Webview (`@application/round-web`)

- **Every reactive, messaging and prop-in concern is Solid's.** The panel's state,
  the command that writes it and the selected scope live in one `createContext`
  provided by `Element/Panel` and read by `Element/Setting`; the overlay registers
  its `mode`/`size` listeners in `onMount` and disposes them with `onCleanup`; the
  scrim - which renders no markup - runs inside an explicit `createRoot` so its
  `pointerdown` listener has an owner to dispose it. Nothing reactive is created
  at module scope: a module signal is a singleton with no owner, and a listener
  registered there can never be removed.
- `Package/Web/Source` is now the house tree, mirroring the crate: `Element/` one
  component per file, `Fn/` one function per file - `Name` and `Describe` (how a
  monitor is named), `Measure` (its resolution), `Locate` (the work area of the
  monitor the panel is on) and `Fit` (size the window, then pull it back inside
  the work area) - `Type/Panel.ts` for the shapes and `Context.ts` for the one
  context. Every element and function is an anonymous `export default`, so a call
  site names what it imports. `Menu.tsx` is eight lines: it mounts `Panel`.
- Identifiers follow the house rule throughout: PascalCase, one word, present
  tense - `Commit` and `Reset` for the two callbacks, `Sync` and `Select` for the
  setters, `Write` for the slider's own value.
- The corner no longer carries a `--Corner` of its own. It inherits the one
  `.Window` sets, which is what lets a per-monitor radius reach the pixels.

---

## 0.0.12

### Crate (`pround`)

- `Crate/Round/Source` is now the house tree: one item per leaf, named by its
  kind, with the path carrying the domain. `Menu/mod.rs`'s 338 lines became
  `Build`, `Open`, `Hide`, `Toggle`, `Place`, `State`, `Monitor` and one leaf per
  command; `Settings`, `Screen`, `Backdrop`, `Message`, `Mode` and `Tray` split
  the same way. `Bounds` (physical-pixel geometry, with rounding in one place),
  `Visibility` (a window's wanted state as a value instead of a `bool`) and
  `Window::Chrome` (the builder flags all three layers share) are new.
- `Overlay`'s three `pub use` lines are gone. The platform backends are declared
  as one `Platform` module with `#[path]`, so a call site reads
  `Overlay::Platform::Art(window)` on every platform and the compiler resolves
  `Platform` to whichever file the target compiled - there is no second name for
  a platform function, and therefore nothing to keep in step.
  `Overlay::Layer::Trait` states the five-operation contract once, so a backend
  that forgets an operation no longer compiles.
- The eight `#[tauri::command]`s are one leaf each and pin their wire name with
  `#[tauri::command(rename = "...")]`, so the functions are PascalCase
  (`Menu::Command::Size::Size`) while the webview keeps invoking `set_size`.
  Every key the panel reads is pinned per field with `#[serde(rename = "...")]`, so
  the payload is byte-identical.
- The tray icon no longer carries a native menu. Every control - the radius for
  all monitors and for each monitor on its own, appearance, show/hide, the dock
  toggle and quit - lives in the Menu window a click on the icon opens, which
  was already doing the job and is now the only way in.
- `Action`, `Effect` and `Settings::next` are gone with the menu they modelled:
  the panel's `#[tauri::command]`s are the command surface, and `Settings` is
  still the only place the values live.
- Any button on the tray icon toggles the Menu window: left, right and middle
  all open it when it is closed and close it when it is open. A right click used
  to reach the native menu; with no menu left there is nothing for it to show,
  so it is the same switch as every other button.
- The panel is dismissed by a click on a backdrop of Round's own, not by an
  inference about focus. A popover has to know when the user left, and the OS
  never says: `WindowEvent::Focused(false)` states that the keyboard moved and a
  pointer position states where the mouse was, neither states what the user did.
  Every guard built on those - a settle window after `show`, an
  `NSApplication.isActive` probe, a scan for another focused window of the app,
  and a pointer-containment test - produced a wrong answer at least once: the
  panel closed under the control being used, then on a click inside the app. All
  four are gone. While the panel is open a transparent, non-focusable backdrop
  window covers each monitor's work area, one level above the corner art and one
  below the panel, and its document invokes a new `dismiss` command on the first
  `pointerdown` of any button. A dismissal is now a click delivered to a window
  of Round's own.
- `Menu` no longer reacts to `WindowEvent::Focused(false)` at all: the settle
  window, the `Blurred`/`Shown` timestamps, `locate`/`Pointer`, `elsewhere`, the
  `NSApplication.isActive` probe and `Overlay::clickable` are deleted.
  `Menu::toggle` keeps its meaning - open if hidden, hide if visible - without
  the guard, and `Menu::hide` closes the panel and the backdrop together, so a
  tray click still toggles the panel shut and `dismiss` is idempotent.
- The backdrop stops at each monitor's work area rather than covering the whole
  screen: the menu bar (the Dock strip on macOS, the taskbar on Windows) stays
  outside it, so a click on the tray icon reaches the icon instead of being
  swallowed by an invisible window. A click in that strip activates whatever it
  landed on and leaves the panel open - the one click the backdrop cannot see,
  and the price of never swallowing the tray. Each backdrop carries its own
  monitor's work area in physical pixels, so a desk that mixes a Retina display
  with a 1x one, or puts a display at a negative origin, needs no arithmetic.
- `Settings::broadcast` skips the backdrop windows. They are hidden and shown by
  the panel, so a broadcast that restored them would put a click-eating window
  over the desktop with the panel closed, once per slider step.
- `Overlay` is now the window ladder rather than a set of one-off lifts:
  `art` / `scrim` / `panel` / `front` / `dock`, with the order spelled out once
  per platform - AppKit levels 25 / 26 / 27 on macOS, a re-insert at the top of
  the topmost band on Windows, the raise order on X11. `raise` and `present` are
  renamed `art` and `front`, and `clickable` is gone with the guard that asked
  it. The panel now sits above the corner art as well as above the backdrop, so
  the corners no longer draw over it.
- A monitor's label is unique, and every stored override survives the change. A
  label is the identity of a display: it names its overlay window, keys its
  per-monitor overrides and values its entry in the panel's scope selector. It
  used to be the display's name with the punctuation stripped, so two displays of
  the same model shared one label - one set of settings between them, one overlay
  window between them (Tauri refuses a second window under a label it already
  holds, so the second display drew no corner art at all) and two selector entries
  that could not be told apart. `Screen::Unique` now settles the set: a name one
  monitor asked for is kept exactly as it was, and a name two or more asked for is
  qualified with that monitor's own position on the desktop, which is a fact about
  the display rather than about the order the system enumerated it in. The index
  is the last resort, for two displays showing one picture. Nothing is renamed out
  from under the store: `Screen::Struct` carries every label a display has
  answered to - its bare name, its qualified one and the `monitor<index>` an
  unnamed display used to take - and `Settings::Migrate::Label` moves the stored
  override onto the label in use on every read, so a twin plugged in or pulled out
  costs nothing either way.

### Webview (`@application/round-web`)

- The panel's monitor is its navigation axis, not a row. A scope selector at the
  top - a native `<select>` holding "All monitors" and then one entry per
  display, each carrying the index the system gave it and the pixels it draws at
  - chooses what the three setting rows below apply to, so the panel draws the
  same rows with one monitor as with a thousand. Each row marks the selected
  monitor's own value and its `↺` drops only that override; with "All monitors"
  selected a row marks how many monitors a write there would drop, and `↺`
  restores the default and clears every override. The selection is spelled out
  under the selector, where it can wrap, because a closed select truncates the
  name that tells two displays apart. `Menu::Monitor` carries the display's
  physical `width`/`height` for that reason alone.
- `backdrop.html` and `Source/Backdrop.ts`: the backdrop's document. It paints
  nothing, has no input and no state, and does one thing - a `pointerdown`
  listener in the capture phase that invokes `dismiss`, for any button. Its
  window is created with `focusable(false)` and `accept_first_mouse(true)`, so
  the click cannot take the keyboard and still counts when Round is inactive.
- `vite.config.ts` builds it as a third entry beside `index.html` and
  `menu.html`.

### Fixed

- **Per-monitor settings now stay per monitor.** Four separate faults stacked on
  the same symptom, so "change this display's ink" moved every display:
  `Emitter::emit` on a window is not scoped to it (the trait's default reaches
  every webview, so a loop over monitors delivered each value to all of them and
  the last write won) - the broadcast uses `emit_to(label, ...)`; the overlay's
  `listen` had no target, which the API resolves to a **global** listener that a
  targeted emit can never match - it now names its window; `Corner.tsx` pinned
  its own `--Corner`, overriding the window's, so the radius never changed - the
  corner inherits it; and every command re-enumerated the monitors and re-resolved
  their labels, which can produce a label no window carries - the set is now
  resolved once at launch (`Screen::Registry`) and read by every command.
- The panel's scope selector was proven to send the monitor's label on every
  command (`set_mode {"label":"Monitor26344","mode":"light"}`), so the selector
  itself needed no change - the faults were all downstream of it.
- `sizes`, `size`, `mode` and `hidden` from the pre-per-monitor store are folded
  into the new shape on load, so an existing install keeps its tuning.

### Tooling

- `cargo test` covers the command path: each test drives a real IPC request into
  the same handler the binary registers (`Commands!`) against Tauri's mock
  runtime, then reads the store the command wrote - command → load → write → save
  → broadcast → state, which no unit test of `Per::Set` can reach.
- `Trace!` records one line per stage of that path in a debug build only. The
  gate is `cfg!(debug_assertions)` **around** the `format!`, so a release build
  carries neither the call nor the format string.
- `turbo` drives `Build`, `dev` and `serve` (`turbo.json`; `pnpm-workspace.yaml`
  already listed `Package/Web`), matching the other PlayForm repositories.

### Dependencies

- `solid-js` pinned back to 1.9.20. 2.0.3 removed the `solid-js/web` entry point
  this code imports and is outside `vite-plugin-solid` 2.x's `^1.7.2` peer
  range, so `pnpm run Build` failed with `"./web" is not exported`.

---

## 0.0.11

### Repository

- One source of truth for the version: `Cargo.toml` →
  `[workspace.package].version`. `Crate/Round/tauri.conf.json` no longer carries
  a `version` field, so the app and its bundles take `CARGO_PKG_VERSION` from
  the workspace manifest, and the two npm manifests - which cannot inherit it -
  are written from that source. `Release.sh` reads the same source and refuses a
  drifted tree instead of offering a tag that disagrees with the crate.

### Tooling

- **`Crate/Maintain`** - a Rust CLI (`Maintain`) that owns the release. It is
  the single writer for the version (`version`, `version set`, `version bump`,
  `version sync`, refusing drift), the local mirror of the CI gates (`check`),
  the bundle build (`build`), and the whole ceremony in one command
  (`release <version> [--publish] [--dry-run]` - bump, commit, tag, push, then
  dispatch the crates.io publish). `publish = false`; it is a tool for this
  repository.
- The throwaway `Scripts/Version.mjs` that briefly held the version logic is
  gone: a Node script that only edited four manifests did not belong in a
  repository whose tooling is Rust, and it could not do anything past the bump.

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
