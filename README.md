<p align="center">
  <picture>
    <source media="(prefers-color-scheme: dark)" srcset="https://PlayForm.Cloud/Image/GitHub/Round/icon.ico?v=2">
    <source media="(prefers-color-scheme: light)" srcset="https://PlayForm.Cloud/Image/GitHub/Round/icon.ico?v=2">
    <img width="72" height="72" alt="Round" src="https://PlayForm.Cloud/Image/GitHub/Round/icon.ico?v=2">
  </picture>
</p>

---

# [Round] ⬜

> [!NOTE]
>
> A transparent, click-through overlay that draws rounded corners on every
> connected monitor. Native on **macOS** and **Windows**; **Linux** builds but
> is untested.
>
> _One tray icon. One overlay per screen. No window manager plugins._

[![release](https://img.shields.io/github/v/release/PlayForm/Round?label=release&color=blue)](https://github.com/PlayForm/Round/releases)
[![crate](https://img.shields.io/static/v1?label=crate&message=pround&color=orange)](https://crates.io/crates/pround)
[![rust](https://img.shields.io/static/v1?label=rust&message=1.90%2B&color=orange)](https://www.rust-lang.org)
[![license](https://img.shields.io/static/v1?label=license&message=CC0-1.0&color=lightgrey)](LICENSE)
[![update](https://img.shields.io/github/last-commit/PlayForm/Round?label=update&color=black)](https://github.com/PlayForm/Round)
[![issue](https://img.shields.io/github/issues/PlayForm/Round?label=issue&color=black)](https://github.com/PlayForm/Round/issues)
[![star](https://img.shields.io/github/stars/PlayForm/Round?label=star&logo=github&color=black)](https://github.com/PlayForm/Round)
[![download](https://img.shields.io/github/downloads/PlayForm/Round/total?label=download&color=black)](https://github.com/PlayForm/Round/releases)

![`Round`](https://PlayForm.Cloud/Image/GitHub/Round/Cover.png?v=2)

---

## What it does

Round is a Tauri 2 tray application. It spawns one borderless, transparent
webview per monitor, sized to that monitor's bounds, draws four rounded corners
inside it with Solid, and is controlled from the system tray. Size, ink, and
visibility are persisted through `tauri-plugin-store`, so the desktop looks the
same after a reboot.

Each window is lifted above the OS chrome, because a corner that the menu bar or
the taskbar draws over is not a corner:

- **macOS** sets `NSStatusWindowLevel`, the collection behaviour
  `canJoinAllSpaces | stationary | ignoresCycle | fullScreenAuxiliary`, and
  `setIgnoresMouseEvents:YES`. Transparency requires `macOSPrivateApi`.
- **Windows** calls `SetWindowPos(HWND_TOPMOST, ..., SWP_NOACTIVATE)` and adds
  `WS_EX_TOOLWINDOW | WS_EX_NOACTIVATE | WS_EX_TRANSPARENT` on top of Tauri's
  `WS_EX_LAYERED`, then switches off Windows 11's own corner rounding
  (`DWMWCP_DONOTROUND`) so the drawn corners are the only ones.
- **Linux** falls back to `always_on_top` + `skip_taskbar`; struts and panels
  from the desktop environment may still draw over the corners.

---

## Install

### Release bundle

Download the macOS `.dmg` or the Windows installer from
[releases](https://github.com/PlayForm/Round/releases) and launch it. Round
lives in the tray - there is no dock window and no main window.

> [!IMPORTANT]
>
> The macOS bundle is ad-hoc signed (`signingIdentity: "-"`) and not notarised,
> because hardened-runtime signing without a Developer ID is rejected on Apple
> Silicon. Gatekeeper asks you to confirm the first launch.

### From source

```sh
git clone https://github.com/PlayForm/Round.git
cd Round
pnpm install
pnpm run Build # vite -> Package/Web/Target
cargo build --release
./Target/release/round
```

To produce the installers instead of the bare binary:

```sh
pnpm tauri build
```

### As a Rust library

The same engine is published as a crate, so an overlay can be added to another
Tauri application without copying this one:

```sh
cargo add pround
```

```rust
use pround::{Overlay, Screen::Screen};

// One overlay per monitor, then lift it above the OS chrome.
let screen = Screen::of(monitor, index);

let window = tauri::WebviewWindowBuilder::new(app, &screen.label, tauri::WebviewUrl::App("index.html".into()))
	.inner_size(screen.width.into(), screen.height.into())
	.position(screen.x.into(), screen.y.into())
	.transparent(true)
	.build()?;

Overlay::raise(&window); // platform-correct lift, no cfg at the call site
```

The crate ships the library target and the `round` binary. The binary embeds the
webview build (`Package/Web/Target`) at compile time, so it is built from a
checkout where `pnpm run Build` has run - a `cargo install pround` without that
directory has no corners to draw and the build says so.

> [!NOTE]
>
> The published `pround` 0.0.1 is a one-version name reservation. The library
> described here ships with 0.0.9, which supersedes it; until that version is
> published, `cargo add pround` resolves the empty reservation.

---

## The tray menu

- **Increase Size** ➕ - grow the corner radius by 6px.
- **Decrease Size** ➖ - shrink it by 6px, never below 0.
- **Reset** ↩️ - back to the default radius of _23px_.
- **Dark** 🌑 / **Light** ☀️ - draw the corners in black or white.
- **Show** 👨🏻 / **Hide** 🥷🏽 - show or hide every overlay at once.
- **No Dock** ⚓ - macOS only; toggles the dock icon through
  `NSApplicationActivationPolicy.Accessory` and is checked while the icon is
  hidden.
- **Exit** ❌ - quit.

Every entry is one variant of `pround::Action::Action`, which owns both the menu
identifier and the menu text; the handler is a match over `Action` and returns
an `Effect` (`Persist`, `Dock(hidden)`, `Exit`) rather than re-deriving what the
click meant.

---

## Repository layout

- **`Crate/Round`** - the Rust crate, published to crates.io as **`pround`**.
  `Source/Library.rs` is the crate root (the overlay engine), `Source/Main.rs`
  is the `round` binary, and each domain (`Action`, `Effect`, `Message`, `Mode`,
  `Overlay`, `Screen`, `Settings`, `Tray`) is its own PascalCase directory with
  a `mod.rs` facade.
- **`Package/Web`** - the webview package: Solid 1.9 + Vite 8 + Tailwind 3,
  built by `@playform/build`'s tsconfig base into `Package/Web/Target`. The
  crate embeds that directory at compile time; it is never copied into the
  crate.
- **`Documentation`** - the guides
  ([`Architecture.md`](Documentation/Architecture.md),
  [`Releasing.md`](Documentation/Releasing.md), an
  [`Index.md`](Documentation/Index.md) map) and the architecture graphs
  (`Knowledge.dot`, `Knowledge.svg`).
- **`Crate/Maintain`** - the release CLI (`maintain`). One writer for the
  version across every manifest, the local mirror of the CI gates, the bundle
  build, and the whole ceremony (`maintain release 0.0.11 --publish`) in one
  command. `publish = false`: it is a tool for this repository, not a crate.
- **`Release.sh`** - the local signed macOS release; the tag it offers comes
  from `maintain version`, so a drifted tree cannot start one.
- **`.github/workflows`** - `Check`, `Build`, `Publish`, `Auto`, `Dependabot`,
  `GitHub`.

---

## Documentation

- **[Architecture](Documentation/Architecture.md)** - the module map, the
  startup sequence, the tray → settings → webview event flow, the webview
  contract, and where to change what.
- **[Releasing](Documentation/Releasing.md)** - one version in `Cargo.toml`, the
  `maintain` CLI that writes it everywhere, the `Round/v*` tag, the opt-in
  crates.io publish, and how to test a publish without burning a version.
- **[Documentation index](Documentation/Index.md)** - the map of the set.

---

## Development

Prerequisites: the Rust **nightly** toolchain from `rust-toolchain.toml`
(rustfmt + clippy included), Node **≥ 22.22.3**, and pnpm **12** (pinned in
`package.json`).

```sh
pnpm install   # workspace install, Package/Web included
pnpm run dev   # vite dev server on :1420
pnpm tauri dev # the app against the dev server
pnpm run Build # production webview build
cargo build    # debug binary at Target/debug/round

# the release CLI (Crate/Maintain)
cargo install --path Crate/Maintain # once, gives you `maintain`
pnpm run Version                    # maintain version: the number + drift check
pnpm run Check                      # maintain check: fmt, clippy -D warnings, check
```

`Cargo.lock` is deliberately untracked: the crate publishes to crates.io and
every consumer resolves its own graph. `Target/` and `Package/Web/Target/` are
build output and are ignored.

---

## Continuous integration

- **`Check.yml`** - on every push and pull request to `Current`: builds the
  webview package once, shares it as an artifact, then runs `cargo fmt --check`,
  `cargo clippy -D warnings`, `cargo check`, and a packaging guard that asserts
  the crate's own files (`Source/`, `icons/`, `tauri.conf.json`) are in the
  published tarball.
- **`Build.yml`** - on a `Round/v*` tag: creates the release once, then bundles
  on macOS (arm64 and x86_64), Linux, and Windows, checksums every artifact,
  attaches them to the release, and fails if a platform is missing.
- **`Publish.yml`** - on a `Round/v*` tag: the packaging guard and a library
  consumer probe run against the crate. Publishing to crates.io is an explicit
  opt-in (`workflow_dispatch` with `publish_crates=true`), so a tag push can
  never burn an immutable version; it authenticates through crates.io trusted
  publishing (OIDC, no stored registry token) and skips a version that is
  already live.
- **`Auto.yml`** - a daily heartbeat that touches `.github/Update.md`, nothing
  else.
- **`Dependabot.yml`** - daily updates for npm, cargo, and GitHub Actions, with
  Tailwind pinned to 3.x.

---

## Changelog

See
[`CHANGELOG.md`](https://github.com/PlayForm/Round/blob/Current/CHANGELOG.md)
for a history of changes to this app.

## License

[CC0-1.0](LICENSE) - public domain dedication.

[Round]: https://github.com/PlayForm/Round
