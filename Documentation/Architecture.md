# Architecture

How Round is put together: two workspace crates, one pnpm package, one process,
one webview per monitor plus the menu window that drives them.

---

## The packages

- **`Crate/Round`** - the Rust crate, published to crates.io as **`pround`**. It
  is both faces of the product: `Source/Library.rs` is the crate root (the
  overlay engine) and `Source/Main.rs` is the `round` binary (the tray app).
  `cargo add pround` gives the engine; `cargo install` from a checkout gives the
  app.
- **`Crate/Maintain`** - the release CLI (`Maintain`), a second workspace member
  that is never published. It owns the version across every manifest, mirrors
  the CI gates, builds the bundle, and cuts the release; see
  [`Releasing.md`](Releasing.md). It is separate from `Crate/Round` because the
  app must not depend on the tool that releases it.
- **`Package/Web`** - the webview package. Solid 1.9 + Vite 8 + Tailwind 3,
  built by `@playform/build`'s tsconfig base into `Package/Web/Target`. The
  crate embeds that directory at compile time (`tauri.conf.json` →
  `frontendDist: "../../Package/Web/Target"`); it is never copied into the
  crate.

The split is not cosmetic: the web package has a Node toolchain and a lockfile,
the crate has a Rust toolchain and none. Keeping them apart is what lets
`cargo add pround` compile the library on a machine that has never run `pnpm`.

---

## Module map

Each domain is a PascalCase directory with a `mod.rs` facade; `Library.rs`
declares them and holds nothing else.

- **`Menu`** - the menu window, and the app's whole control surface. It builds a
  borderless, transparent, always-on-top webview (`menu.html`) under the tray
  icon, toggles it on a click, dismisses it when it really loses focus, and
  exposes the panel's commands - `state`, `set_size`, `set_mode`, `set_hidden`,
  `set_dock`, `reset`, `quit` - each of which returns the state to render.
  "Really" is three guards on the blur: a window that is already hidden, a blur
  inside the settle window after `show`, and a blur that only moved the keyboard
  to another window of Round are all ignored; only a blur with focus outside the
  app (`NSApplication.isActive` still true) is a dismissal. Without the third
  guard every command would close the panel it was clicked in, because a command
  ends in a broadcast over the overlay windows and showing a window can key it.
- **`Mode`** - the two inks a corner can be drawn in. Serialises to the
  lowercase word the webview reads (`dark`, `light`); unknown store values fall
  back to the default.
- **`Message`** - the wire pair pushed to every window: `Message::Mode(Mode)` /
  `Message::Size(i64)` inside `Payload { message }`, externally tagged so the
  frontend reads `payload.message.Size`.
- **`Settings`** - the persisted state (`size`, `mode`, `hidden`,
  `dock_hidden`), its store file (`settings.json`), the `window.settings` init
  script, per-window visibility, and the broadcast to every window.
- **`Screen`** - one monitor resolved to the window label and the logical bounds
  a webview window takes. Labels keep ASCII alphanumerics only (Tauri rejects
  the rest); an unnamed monitor becomes `monitor<index>`.
- **`Overlay`** - lifting a transparent webview above the OS chrome. Exactly one
  `raise`/`present`/`dock` trio is re-exported per platform, so call sites are
  identical everywhere and there is no runtime branch to get wrong:
    - `MacOS.rs` - `NSStatusWindowLevel` (25), collection behaviour
      `canJoinAllSpaces | stationary | ignoresCycle | fullScreenAuxiliary`,
      `setIgnoresMouseEvents:YES`, `orderFrontRegardless` for `present`, and
      `setActivationPolicy` for the dock toggle.
    - `Windows.rs` - `SetWindowPos(HWND_TOPMOST, ..., SWP_NOACTIVATE)` plus
      `WS_EX_TOOLWINDOW | WS_EX_NOACTIVATE | WS_EX_TRANSPARENT` on top of
      Tauri's `WS_EX_LAYERED`, and `DWMWCP_DONOTROUND` so Windows 11 does not
      draw a second set of corners.
    - `Linux.rs` - nothing beyond the builder's `always_on_top` +
      `skip_taskbar`; a compositor may still draw panels over the corners.
  `present` is the one place a window is put on screen, and it is deliberately
  not `WebviewWindow::show`: that forwards to tao's `set_visible(true)`, which
  calls `makeKeyAndOrderFront:` and makes the overlay the key window. An overlay
  never receives a click, so taking the keyboard from the Menu panel buys
  nothing and costs the panel its open state; `orderFrontRegardless` orders the
  window front without the key change (and still works while the app is
  inactive, where `makeKeyAndOrderFront:` does nothing).
- **`Tray`** - the tray icon and its click, nothing else. No native menu is
  attached: a click calls `Menu::toggle`, so the tray owns no command, no label
  and no state.

The graph in [`Knowledge.dot`](Knowledge.dot) (rendered to `Knowledge.svg`) is
the same map with the edges drawn.

---

## Startup

1. `tauri-plugin-store` opens `settings.json` under the app config dir.
2. `Settings::load` reads the four keys, falling back per field to the default
   (`size` 23, `mode` dark, `hidden` false, `dock_hidden` false), then `save`
   writes the result back - which is how a fresh install gets its defaults.
3. macOS: `Overlay::dock(!settings.dock_hidden)` applies the saved activation
   policy before any window exists.
4. For every monitor: build a borderless, transparent, non-focusable,
   non-resizable, always-on-top webview at the monitor's logical bounds, run the
   init script (`window.settings = {"size":23,"mode":"dark"};`), build it
   hidden, `Overlay::raise` it, then `Settings::visible` it - which `present`s it
   unless the saved state is hidden.
5. `Tray::install` builds the icon and wires its click to `Menu::toggle`. The
   menu window itself is built on the first click, not at startup, so a launch
   that never opens the panel never pays for it.

---

## Event flow

```
tray click
  -> Menu::toggle(app, rect)
       hidden -> place under the icon, show, focus
       visible -> hide
  -> the panel invokes a command: state / set_size / set_mode /
     set_hidden / set_dock / reset / quit
  -> Settings::load(&store) -> mutate -> Settings::save(&store)
  -> broadcast: for every overlay window emit "size", emit "mode", and apply
     visibility only where it differs from `is_visible()` - a window already in
     the wanted state is left alone, because showing it is not a no-op
  -> the command returns the new State, which the panel renders
```

The webview is deliberately dumb: it holds no state of its own beyond two
signals initialised from `window.settings` and updated by those events. All
decisions - including the reset value and the clamp - live in Rust.

---

## The webview contract

- `window.settings = { size: <number>, mode: "dark" | "light" }` is injected as
  the window's initialisation script, so the first paint already has the right
  size and ink instead of flashing the defaults.
- `listen("size", ({ payload }) => payload.message.Size)` and
  `listen("mode", ({ payload }) => payload.message.Mode)` are the only inbound
  channels; both are externally tagged serde enums, which is why the payload key
  is capitalised.
- `data-mode` on the `.Window` element switches the corner ink
  (`.Window[data-mode="light"] .Corner` paints white, the default is black).
- `--Corner` is set inline per corner from the size signal; `Corner.css` derives
  width, height and the radial-gradient mask from it.
- `data-corner` (`BottomLeft`, `BottomRight`, `TopLeft`, `TopRight`) positions
  and rotates the four corners.

Both `data-mode` and `--Corner` are emitted inside Solid render effects that
read the signal accessors, so the panel's changes repaint live.

---

## Where to change what

- **A new control** - a `#[tauri::command]` on `Menu` that mutates `Settings`
  and returns the `State` the panel should render, its entry in the
  `invoke_handler` in `Main.rs`, and the row in `Package/Web/Source/Menu.tsx`
  that calls it. Add the field to `Menu::State` when the panel has to display
  it.
- **A new persisted setting** - a field on `Settings`, its key in `load`/`save`,
  and (if the webview needs it) a field in `window.settings` plus the matching
  TypeScript type in `Package/Web/Source/Type/Settings.ts`.
- **A platform behaviour** - only the matching file under `Source/Overlay/`; the
  re-export in `Overlay/mod.rs` means no call site changes.
- **A corner's look** - `Package/Web/Source/Asset/CSS/Corner.css` and the
  `.Window` markup in `Source/Index.tsx`; the Rust side only carries numbers.
