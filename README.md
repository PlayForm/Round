<table><tr> <td colspan="1"> <h3 align="center"> <picture> <source media="(prefers-color-scheme: dark)" srcset="https://PlayForm.Cloud/Image/GitHub/Round/32x32.png"> <source media="(prefers-color-scheme: light)" srcset="https://PlayForm.Cloud/Image/GitHub/Round/32x32.png"> <img width="28" alt="" src="https://PlayForm.Cloud/Image/GitHub/Round/32x32.png"> </picture>  </h3> </td> <td colspan="3" valign="top"> <h3 align="center"> Round </h3> </td> </tr><tr><td valign="top" colspan="2"><a href="https://GitHub.Com/PlayForm/Round" target="_blank"> <picture> <source media="(prefers-color-scheme: dark)" srcset="https://img.shields.io/github/last-commit/PlayForm/Round?label=Update&color=black&labelColor=black&logoColor=white&logoWidth=0"> <source media="(prefers-color-scheme: light)" srcset="https://img.shields.io/github/last-commit/PlayForm/Round?label=Update&color=white&labelColor=white&logoColor=black&logoWidth=0"> <img src="https://img.shields.io/github/last-commit/PlayForm/Round?label=Update&color=black&labelColor=black&logoColor=white&logoWidth=0" alt="Update" title="Update"> </picture> </a><br><a href="https://GitHub.Com/PlayForm/Round" target="_blank"> <picture> <source media="(prefers-color-scheme: dark)" srcset="https://img.shields.io/github/issues/PlayForm/Round?label=Issue&color=black&labelColor=black&logoColor=white&logoWidth=0"> <source media="(prefers-color-scheme: light)" srcset="https://img.shields.io/github/issues/PlayForm/Round?label=Issue&color=white&labelColor=white&logoColor=black&logoWidth=0"> <img src="https://img.shields.io/github/issues/PlayForm/Round?label=Issue&color=black&labelColor=black&logoColor=white&logoWidth=0" alt="Issue" title="Issue"> </picture> </a><br></td><td valign="top" colspan="2"><a href="https://GitHub.Com/PlayForm/Round" target="_blank"><picture><source media="(prefers-color-scheme: dark)" srcset="https://img.shields.io/github/stars/PlayForm/Round?style=flat&label=Star&logo=github&color=black&labelColor=black&logoColor=white&logoWidth=0"><source media="(prefers-color-scheme: light)" srcset="https://img.shields.io/github/stars/PlayForm/Round?style=flat&label=Star&logo=github&color=white&labelColor=white&logoColor=black&logoWidth=0"><img src="https://img.shields.io/github/stars/PlayForm/Round?style=flat&label=Star&logo=github&color=black&labelColor=black&logoColor=white&logoWidth=0" alt="Star"></picture></a><br><a href="https://GitHub.Com/PlayForm/Round" target="_blank"> <picture> <source media="(prefers-color-scheme: dark)" srcset="https://img.shields.io/github/downloads/PlayForm/Round/total?label=Download&color=black&labelColor=black&logoColor=white&logoWidth=0"> <source media="(prefers-color-scheme: light)" srcset="https://img.shields.io/github/downloads/PlayForm/Round/total?label=Download&color=white&labelColor=white&logoColor=black&logoWidth=0"> <img src="https://img.shields.io/github/downloads/PlayForm/Round/total?label=Download&color=black&labelColor=black&logoColor=white&logoWidth=0" alt="Download" title="Download"> </picture> </a><br><a href="https://GitHub.Com/PlayForm/Round" target="_blank"><b>Round ⬜</b></a></td></tr></table>

<p align="center">
  <img width="64" height="64" src="https://PlayForm.Cloud/Image/GitHub/Round/icon.ico?v=2" alt="Round" />
</p>

# Round

Rounds the corners of every monitor - natively on **macOS** and **Windows**.
Should also build and run on **Linux** (X11 / Wayland), but that path is
currently untested - bug reports and patches are welcome.

![`Round`](https://PlayForm.Cloud/Image/GitHub/Round/Cover.png?v=2)

## Getting started

`Round` is a tray application built on Tauri 2. It creates a transparent,
click-through overlay window on every connected monitor, draws four rounded
corners on each, and is controlled from the system tray (menu bar on macOS,
notification area on Windows).

The overlay is lifted above OS chrome so the corners cover the screen edges
completely:

- **macOS** - the windows sit at `NSStatusWindowLevel`, ride along across all
  Spaces, and survive other apps entering fullscreen, so the corners overlay the
  menu bar at the top of every display. Transparency uses `macOSPrivateApi`.
- **Windows** - the windows are pushed to `HWND_TOPMOST` after creation and
  flagged `WS_EX_TOOLWINDOW | WS_EX_NOACTIVATE | WS_EX_TRANSPARENT`, so the
  corners overlay the taskbar without stealing focus or showing in Alt-Tab.
- **Linux** - falls back to Tauri's default `always_on_top` + `skip_taskbar`,
  no platform-specific lifting. Untested in practice; on most compositors the
  overlay will sit above normal windows but may render under panels/docks
  configured as struts. PRs welcome.

Clicks always pass through, the overlays never take focus, and settings persist
across launches.

## Dependencies

The Rust side uses:

- `tauri` 2 with the `tray-icon` and `macos-private-api` features
- `tauri-plugin-store` - persistent key-value store for the size, mode, and
  visibility settings
- `regex` - sanitizes monitor names into valid window labels
- `serde` / `serde_json` - payload (de)serialization
- `objc2` (macOS only) - raises each window above the menu bar
- `windows-sys` (Windows only) - raises each window above the taskbar

The frontend is Solid 1.9 + Vite 8 with `@tauri-apps/api` 2, bundled by
`@playform/build`.

## Options

The app has several menu items:

- **Increase Size** ➕
- **Decrease Size** ➖
- **Reset** ↩️
- **Dark** 🌑
- **Light** ☀️
- **Show** 👨🏻
- **Hide** 🥷🏽
- **Exit** ❌

Clicking on **Increase Size** or **Decrease Size** increases or decreases the
roundness of the windows, respectively.

**Reset** sets the corner radius back to the default value of _23px_.

**Dark** and **Light** switch the app between dark and light mode.

**Show** and **Hide** show or hide all windows, respectively.

**Exit** closes the app.

## Changelog

See [`CHANGELOG.md`](CHANGELOG.md) for a history of changes to this app.
