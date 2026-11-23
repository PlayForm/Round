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
  icon, opens the `Backdrop` scrims under it, toggles it on a click, and exposes
  the panel's commands - `state`, `set_size`, `set_mode`, `set_hidden`,
  `set_dock`, `reset`, `dismiss`, `quit` - each of which returns the state to
  render.
  It does not decide its own dismissal. A popover has to know when the user
  left, and the OS never says: a blur states that the keyboard moved, a pointer
  position states where the mouse was, and neither states what the user did. The
  panel used to read `WindowEvent::Focused(false)` behind four guards - a settle
  window after `show`, an `NSApplication.isActive` probe, a scan for another
  focused window of the app, and a pointer-containment test - and every one of
  them produced a wrong answer at least once: the panel closed under the control
  being used, then on a click inside the app. All four are gone. What is left is
  `dismiss`, invoked by the scrim's own document when a pointer goes down on it,
  and `toggle` from the tray icon. Nothing in this module reads focus,
  activation, a timer or geometry.
- **`Backdrop`** - the scrim. One borderless, transparent, non-focusable webview
  per monitor (`backdrop.html`), covering that monitor's work area one level
  above the art and below the panel. It exists only while the panel is open:
  `Menu::open` builds and shows it, `Menu::hide` closes it, and `dismiss` hides
  both together. Its document is empty except for one `pointerdown` listener
  that invokes `dismiss` - any button, capture phase - so the dismissal is a
  click delivered to a window of Round's own rather than an inference about one.
  The work area, not the whole rect, is what keeps the tray usable: the menu bar
  (and, on macOS, the Dock strip; on Windows, the taskbar) is outside the scrim,
  so a click there reaches the OS chrome and the tray icon still toggles the
  panel closed. A click in that strip therefore does not dismiss the panel - it
  activates whatever it landed on - which is the one shape of click the scrim
  cannot see, and the price of never swallowing the tray.
  Each scrim carries its own monitor's work area in physical pixels, so a desk
  that mixes a Retina display with a 1x one, or puts a display at a negative
  origin, needs no arithmetic. `Backdrop::owns` is what the broadcast asks
  before treating a window as art: the scrims are hidden and shown by the panel
  and never restored by a broadcast, or a slider step would put a click-eating
  window back on the desktop with the panel closed.
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
- **`Overlay`** - the window ladder, and the one place it is spelled out.
  Exactly one `art`/`scrim`/`panel`/`front`/`dock` set is re-exported per
  platform, so call sites are identical everywhere and there is no runtime
  branch to get wrong. Three layers, bottom to top:
    - **art** - the corner overlay, one window per monitor. Click-through, so a
      click on it is a click on whatever is under it.
    - **scrim** - the backdrop. It takes the mouse, which is the whole point of
      it.
    - **panel** - the Menu window, above the scrim, owning its own clicks.
  On macOS the order is a level: `NSStatusWindowLevel` (25) for the art, 26 for
  the scrim, 27 for the panel. A level is a fact about the window rather than a
  fact about the last call, so re-showing a scrim can never lift it over the
  panel. The art also takes the all-spaces collection behaviour
  (`canJoinAllSpaces | stationary | ignoresCycle | fullScreenAuxiliary`) and
  `setIgnoresMouseEvents:YES`; the scrim adds `stationary | ignoresCycle` to
  whatever it already carries - keeping a full-screen invisible window out of
  Mission Control and out of ⌘` - and deliberately **not** `canJoinAllSpaces`,
  because the panel does not join every Space either: a scrim that did would
  outlive the panel the moment the user switched Space.
  On Windows the order is the topmost band: the art adds
  `WS_EX_TOOLWINDOW | WS_EX_NOACTIVATE | WS_EX_TRANSPARENT` and
  `DWMWCP_DONOTROUND`, the scrim adds `WS_EX_TOOLWINDOW | WS_EX_NOACTIVATE` and
  never the transparent one, and the panel is re-inserted with
  `SetWindowPos(HWND_TOPMOST, ..., SWP_NOACTIVATE)` after the scrims are up -
  which is what puts it on top however many times it has been opened.
  On Linux nothing lifts a window above the compositor's own panels; the scrim
  and then the panel are raised with `set_always_on_top(true)`, and a compositor
  that refuses to stack one client over another leaves the scrim under the
  panel, where a click on the panel is still the panel's.
  `front` is the one place a window is put on screen, and it is deliberately not
  `WebviewWindow::show`: that forwards to tao's `set_visible(true)`, which calls
  `makeKeyAndOrderFront:` and makes the window key. Neither the art nor the
  scrim has any use for the keyboard, and taking it would pull focus out of
  whatever the user was using; `orderFrontRegardless` orders the window front
  without the key change (and still works while the app is inactive, where
  `makeKeyAndOrderFront:` does nothing).
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
   hidden, `Overlay::art` it, then `Settings::visible` it - which `front`s it
   unless the saved state is hidden.
5. `Tray::install` builds the icon and wires its click to `Menu::toggle`. The
   panel and the scrims are built on the first click, not at startup, so a
   launch that never opens the panel never pays for them.

---

## Event flow

```
tray click
  -> Menu::toggle(app, rect)
       hidden -> place under the icon, open the scrims, show, raise the panel
                 above them, focus
       visible -> hide the panel and the scrims
  -> the panel invokes a command: state / set_size / set_mode /
     set_hidden / set_dock / reset / quit
  -> Settings::load(&store) -> mutate -> Settings::save(&store)
  -> broadcast: for every ART window emit "size", emit "mode", and apply
     visibility only where it differs from `is_visible()` - a window already in
     the wanted state is left alone, because showing it is not a no-op. The
     panel and the scrims are skipped: the panel renders the state the command
     returns, and the scrim has no state at all
  -> the command returns the new State, which the panel renders

pointerdown on a scrim
  -> invoke("dismiss")
  -> Menu::dismiss: hide the panel and every scrim
```

There is no other way into a dismissal. No focus event, activation probe, timer
or geometry test decides one, so none of them can get it wrong: the panel closes
when a click lands on Round's own scrim, when the tray icon is clicked, or when
the app quits.

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

The backdrop's document has no contract of its own beyond one inbound event: a
`pointerdown` on it invokes `dismiss`. It reads no state, renders nothing and
holds no signals, which is what lets its window stay invisible.

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
- **The order the three layers sit in** - the const block at the top of each
  file under `Source/Overlay/`: the AppKit levels on macOS, the topmost insert
  on Windows, the raise order on X11. `art`, `scrim` and `panel` are the only
  places a level is chosen.
- **A corner's look** - `Package/Web/Source/Asset/CSS/Corner.css` and the
  `.Window` markup in `Source/Index.tsx`; the Rust side only carries numbers.
