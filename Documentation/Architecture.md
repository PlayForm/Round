# Architecture

How Round is put together: two workspace crates, one pnpm package, one process,
one webview per monitor.

---

## The packages

- **`Crate/Round`** - the Rust crate, published to crates.io as **`pround`**. It
  is both faces of the product: `Source/Library.rs` is the crate root (the
  overlay engine) and `Source/Main.rs` is the `round` binary (the tray app).
  `cargo add pround` gives the engine; `cargo install` from a checkout gives the
  app.
- **`Crate/Maintain`** - the release CLI (`maintain`), a second workspace member
  that is never published. It owns the version across every manifest, mirrors the
  CI gates, builds the bundle, and cuts the release; see
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

- **`Action`** - every command the tray can send. One variant per menu entry,
  owning both its identifier (`increase`) and its label
  (`Increase Size\u{2001}➕`), plus `Action::of(id)` derived from a single
  `Every` array so the lookup cannot drift from the menu.
- **`Effect`** - what the caller does after an action is applied: `Persist`,
  `Dock(hidden)`, `Exit`. A value, not a flag: the tray matches on it instead of
  re-deriving intent from the action id.
- **`Mode`** - the two inks a corner can be drawn in. Serialises to the
  lowercase word the webview reads (`dark`, `light`); unknown store values fall
  back to the default.
- **`Message`** - the wire pair pushed to every window: `Message::Mode(Mode)` /
  `Message::Size(i64)` inside `Payload { message }`, externally tagged so the
  frontend reads `payload.message.Size`.
- **`Settings`** - the persisted state (`size`, `mode`, `hidden`,
  `dock_hidden`), its store file (`settings.json`), `next(action)` → the new
  state plus an `Effect`, the `window.settings` init script, per-window
  visibility, and the broadcast to every window.
- **`Screen`** - one monitor resolved to the window label and the logical bounds
  a webview window takes. Labels keep ASCII alphanumerics only (Tauri rejects
  the rest); an unnamed monitor becomes `monitor<index>`.
- **`Overlay`** - lifting a transparent webview above the OS chrome. Exactly one
  `raise`/`dock` pair is re-exported per platform, so call sites are identical
  everywhere and there is no runtime branch to get wrong:
    - `MacOS.rs` - `NSStatusWindowLevel` (25), collection behaviour
      `canJoinAllSpaces | stationary | ignoresCycle | fullScreenAuxiliary`,
      `setIgnoresMouseEvents:YES`, and `setActivationPolicy` for the dock
      toggle.
    - `Windows.rs` - `SetWindowPos(HWND_TOPMOST, ..., SWP_NOACTIVATE)` plus
      `WS_EX_TOOLWINDOW | WS_EX_NOACTIVATE | WS_EX_TRANSPARENT` on top of
      Tauri's `WS_EX_LAYERED`, and `DWMWCP_DONOTROUND` so Windows 11 does not
      draw a second set of corners.
    - `Linux.rs` - nothing beyond the builder's `always_on_top` +
      `skip_taskbar`; a compositor may still draw panels over the corners.
- **`Tray`** - the tray icon, its menu, and the event handler. The menu is built
  from `Action`'s own identifiers and labels; the handler is one match over
  `Effect`.

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
   hidden, `Overlay::raise` it, then show it unless the saved state is hidden.
5. `Tray::install` builds the menu and attaches the handler.

---

## Event flow

```
tray click
  -> Action::of(event.id)
  -> Settings::load(&store).next(action)   ->  (Settings, Effect)
  -> Settings::save(&store)
  -> match Effect
       Persist    -> broadcast
       Dock(x)    -> Overlay::dock(!x) + broadcast
       Exit       -> app.exit(0)
  -> broadcast: for every window emit "size", emit "mode", apply visibility
```

The webview is deliberately dumb: it holds no state of its own beyond two
signals initialised from `window.settings` and updated by those events. All
decisions - including the size step and the reset value - live in Rust.

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
read the signal accessors, so the tray's changes repaint live.

---

## Where to change what

- **A new tray command** - one `Action` variant, its `id()`/`label()` arms, and
  an arm in `Settings::next`. If it has a side effect the settings cannot
  express, add an `Effect` variant and match it in `Tray::install`.
- **A new persisted setting** - a field on `Settings`, its key in `load`/`save`,
  and (if the webview needs it) a field in `window.settings` plus the matching
  TypeScript type in `Package/Web/Source/Type/Settings.ts`.
- **A platform behaviour** - only the matching file under `Source/Overlay/`; the
  re-export in `Overlay/mod.rs` means no call site changes.
- **A corner's look** - `Package/Web/Source/Asset/CSS/Corner.css` and the
  `.Window` markup in `Source/Index.tsx`; the Rust side only carries numbers.
