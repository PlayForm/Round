<table><tr> <td colspan="1"> <h3 align="center"> <picture> <source media="(prefers-color-scheme: dark)" srcset="https://PlayForm.Cloud/Image/GitHub/Round/32x32.png"> <source media="(prefers-color-scheme: light)" srcset="https://PlayForm.Cloud/Image/GitHub/Round/32x32.png"> <img width="28" alt="" src="https://PlayForm.Cloud/Image/GitHub/Round/32x32.png"> </picture>  </h3> </td> <td colspan="3" valign="top"> <h3 align="center"> Round </h3> </td> </tr><tr><td valign="top" colspan="2"><a href="https://GitHub.Com/PlayForm/Round" target="_blank"> <picture> <source media="(prefers-color-scheme: dark)" srcset="https://img.shields.io/github/last-commit/PlayForm/Round?label=Update&color=black&labelColor=black&logoColor=white&logoWidth=0"> <source media="(prefers-color-scheme: light)" srcset="https://img.shields.io/github/last-commit/PlayForm/Round?label=Update&color=white&labelColor=white&logoColor=black&logoWidth=0"> <img src="https://img.shields.io/github/last-commit/PlayForm/Round?label=Update&color=black&labelColor=black&logoColor=white&logoWidth=0" alt="Update" title="Update"> </picture> </a><br><a href="https://GitHub.Com/PlayForm/Round" target="_blank"> <picture> <source media="(prefers-color-scheme: dark)" srcset="https://img.shields.io/github/issues/PlayForm/Round?label=Issue&color=black&labelColor=black&logoColor=white&logoWidth=0"> <source media="(prefers-color-scheme: light)" srcset="https://img.shields.io/github/issues/PlayForm/Round?label=Issue&color=white&labelColor=white&logoColor=black&logoWidth=0"> <img src="https://img.shields.io/github/issues/PlayForm/Round?label=Issue&color=black&labelColor=black&logoColor=white&logoWidth=0" alt="Issue" title="Issue"> </picture> </a><br></td><td valign="top" colspan="2"><a href="https://github.com/PlayForm/Round" target="_blank"><picture><source media="(prefers-color-scheme: dark)" srcset="https://img.shields.io/github/stars/PlayForm/Round?style=flat&label=Star&logo=github&color=black&labelColor=black&logoColor=white&logoWidth=0"><source media="(prefers-color-scheme: light)" srcset="https://img.shields.io/github/stars/PlayForm/Round?style=flat&label=Star&logo=github&color=white&labelColor=white&logoColor=black&logoWidth=0"><img src="https://img.shields.io/github/stars/PlayForm/Round?style=flat&label=Star&logo=github&color=black&labelColor=black&logoColor=white&logoWidth=0" alt="Star"></picture></a><br><a href="https://GitHub.Com/PlayForm/Round" target="_blank"> <picture> <source media="(prefers-color-scheme: dark)" srcset="https://img.shields.io/github/downloads/PlayForm/Round/total?label=Download&color=black&labelColor=black&logoColor=white&logoWidth=0"> <source media="(prefers-color-scheme: light)" srcset="https://img.shields.io/github/downloads/PlayForm/Round/total?label=Download&color=white&labelColor=white&logoColor=black&logoWidth=0"> <img src="https://img.shields.io/github/downloads/PlayForm/Round/total?label=Download&color=black&labelColor=black&logoColor=white&logoWidth=0" alt="Download" title="Download"> </picture> </a><br><a href="https://github.com/PlayForm/Round" target="_blank"><b>Round ⬜</b></a></td></tr></table>

<p align="center">
  <img width="64" height="64" src="https://PlayForm.Cloud/Image/GitHub/Round/icon.ico?v=2" alt="Round" />
</p>

# Round ⬜

A transparent, click-through overlay that draws rounded corners on every
connected monitor. Native on **macOS** and **Windows**; **Linux** should build
but is untested.

![`Round`](https://PlayForm.Cloud/Image/GitHub/Round/Cover.png?v=2)

## Getting started

`Round` ⬜ is a Tauri 2 tray app. It spawns one borderless, transparent webview
per monitor sized to the monitor's bounds, draws four rounded corners with
Solid, and is controlled from the system tray. State (size, theme, hidden,
no-dock) is persisted via `tauri-plugin-store`.

Each window is lifted above OS chrome so the corners cover the full screen:

- **macOS** sets `NSStatusWindowLevel`, the collection behaviour
  `canJoinAllSpaces | stationary | ignoresCycle | fullScreenAuxiliary`, and
  `setIgnoresMouseEvents:YES`. Transparency requires `macOSPrivateApi`.
- **Windows** calls `SetWindowPos(HWND_TOPMOST, ..., SWP_NOACTIVATE)` and adds
  `WS_EX_TOOLWINDOW | WS_EX_NOACTIVATE | WS_EX_TRANSPARENT` on top of Tauri's
  `WS_EX_LAYERED`.
- **Linux** falls back to `always_on_top` + `skip_taskbar`; struts/panels may
  still draw over the corners.

## Dependencies

- `tauri` 2 with the `tray-icon` and `macos-private-api` features
- `tauri-plugin-store` for persisting size, theme, and visibility
- `regex`, `serde`, `serde_json`
- `objc2` (macOS only) - lifts windows above the menu bar and toggles the dock
  activation policy
- `windows-sys` (Windows only) - lifts windows above the taskbar

Frontend: Solid 1.9 + Vite 8 + `@tauri-apps/api` 2, bundled by
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
- **No Dock** ⚓
- **Exit** ❌

Clicking on **Increase Size** or **Decrease Size** increases or decreases the
roundness of the windows, respectively.

**Reset** sets the corner radius back to the default value of _23px_.

**Dark** and **Light** switch the app between dark and light mode.

**Show** and **Hide** show or hide all windows, respectively.

**No Dock** (macOS only) toggles the dock icon via
`NSApplicationActivationPolicy.Accessory`; the menu item is checked when the
dock icon is hidden.

**Exit** closes the app.

## Changelog

See [`CHANGELOG.md`](CHANGELOG.md) for a history of changes to this app.
