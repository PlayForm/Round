# 0.0.9

- macOS native: overlays sit at `NSStatusWindowLevel` with
  `canJoinAllSpaces | stationary | ignoresCycle | fullScreenAuxiliary`
  collection behaviour and `setIgnoresMouseEvents:YES`, covering the menu bar
  across Spaces and fullscreen apps
- Windows taskbar overlay:
  `SetWindowPos(HWND_TOPMOST, ..., SWP_NOACTIVATE)` after creation, plus
  ex-styles `WS_EX_TOOLWINDOW | WS_EX_NOACTIVATE | WS_EX_TRANSPARENT`;
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
