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

## How the tree is shaped

The module tree **is** the API. There is no facade that renames anything:

- **A leaf exports one item, named by its kind.** `Struct`, `Enum`, `Fn`,
  `Trait` or `Const`, in a file of that name. `Menu/State.rs` holds
  `pub struct Struct`; `Menu/Place.rs` holds `pub fn Fn`. The directory chain
  says what the item is for, so a consumer reads `Menu::Place::Fn` and knows
  which function it is without opening the file.
- **A role with one item is a file, not a folder.** `Backdrop/Prefix.rs` holds
  `pub const Const:&str = "backdrop"`, read as `Backdrop::Prefix::Const`. A
  folder exists when a role has several leaves (`Menu/Command/`) or groups
  sub-roles (`Overlay/Layer/`).
- **`mod.rs` declares its children and nothing else.** No items, no re-exports,
  no logic. The only attributes a `mod.rs` carries are the `#[cfg]`/`#[path]`
  pair that select a platform backend.
- **No `pub use`, anywhere.** A re-export is a second name for one thing, which
  is exactly what this shape exists to prevent. Where a call site wants a short
  name it aliases at the `use`: `use crate::Settings::Struct::Struct as
  Settings;`.
- **PascalCase everywhere** - fields, locals, parameters and generic parameters
  as well as types (`pub Size:i64`, `let RequiresFocus = ...`,
  `fn Fn(Option:ExecutionOption)`), with acronyms fully cased.

`Library.rs` declares the twelve domains and holds nothing else.

---

## Module map

- **`Backdrop`** - the scrim. One borderless, transparent, non-focusable webview
  per monitor (`backdrop.html`), covering that monitor's work area one level
  above the art and below the panel. It exists only while the panel is open:
  `Menu::Open::Fn` builds and shows it, `Menu::Hide::Fn` closes it, and
  `Menu::Command::Dismiss::Dismiss` hides both together. Its document is empty
  except for one `pointerdown` listener that invokes `dismiss` - any button,
  capture phase - so the dismissal is a click delivered to a window of Round's
  own rather than an inference about one.
  The work area, not the whole rect, is what keeps the tray usable: the menu bar
  (and, on macOS, the Dock strip; on Windows, the taskbar) is outside the scrim,
  so a click there reaches the OS chrome and the tray icon still toggles the
  panel closed. A click in that strip therefore does not dismiss the panel - it
  activates whatever it landed on - which is the one shape of click the scrim
  cannot see, and the price of never swallowing the tray.
  Each scrim carries its own monitor's work area in physical pixels, so a desk
  that mixes a Retina display with a 1x one, or puts a display at a negative
  origin, needs no arithmetic. `Backdrop::Owns::Fn` is what the broadcast asks
  before treating a window as art: the scrims are hidden and shown by the panel
  and never restored by a broadcast, or a slider step would put a click-eating
  window back on the desktop with the panel closed.
  Its leaves are `Build` (the window), `Show` (place, size, raise one monitor's
  scrim), `Open` (one per monitor, plus the reconciliation that hides a scrim
  whose display went away), `Close`, `Owns`, `Label` (index → label) and
  `Prefix` (the label prefix).
- **`Bounds`** - physical-pixel geometry, shared by everything that places a
  window. `Struct` is a rect in physical `f64`; `Rect` converts the tray icon's
  rect from whichever unit the platform reported, `Monitor` reads a work area,
  `Outer` reads a window's own outer size, and `Position`/`Size` turn the `f64`
  rect into the whole-pixel forms `set_position`/`set_size` take. Rounding
  happens in exactly those two leaves, so a panel that kept a fractional value
  through its centring arithmetic lands on the same pixel it always did.
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
  and `Toggle` from the tray icon. Nothing in this module reads focus,
  activation, a timer or geometry.
  - `Toggle`, `Open`, `Hide`, `Build`, `Place` - the window's life.
  - `State` - the struct the panel renders, one `Serialize` field per control.
  - `Monitor` - one entry in the panel's scope selector: its label and name, the
    physical pixels that tell two displays of the same name apart, and its three
    settings with the origin of each.
  - `Label`, `Width`, `Height`, `Gap` - the window label and the three
    measurements placement needs.
  - `Store` - `Load` (settings as they are on disk) and `Commit` (save, then
    broadcast, then answer). Every command ends in `Commit`, so no command can
    save without broadcasting or broadcast without having saved.
  - `Command` - the eight `#[tauri::command]`s, one leaf each.
- **`Message`** - the wire pair pushed to every window: `Message::Enum::Mode(Mode)`
  / `Message::Enum::Size(i64)` inside `Payload { Message }`, externally tagged so
  the frontend reads `payload.message.Size`. The envelope's key is pinned with
  `#[serde(rename = "message")]`, because the field is `Message` and the wire key
  is lowercase.
- **`Mode`** - the two inks a corner can be drawn in. `Enum` is the value,
  `Word` is the one place the lowercase spelling the webview reads (`dark`,
  `light`) is decided, and `Of` reads one back from the store, falling back to
  the default for anything unrecognised.
- **`Overlay`** - the window ladder, and the one place it is spelled out.
  Three layers, bottom to top:
  - **art** - the corner overlay, one window per monitor. Click-through, so a
    click on it is a click on whatever is under it.
  - **scrim** - the backdrop. It takes the mouse, which is the whole point of
    it.
  - **panel** - the Menu window, above the scrim, owning its own clicks.

  The contract is one `Trait` leaf and one backend per platform, and the
  platform is selected with `#[path]` rather than re-exported:

  ```rust
  #[cfg(target_os = "macos")]
  #[path = "Platform/MacOS/Struct.rs"]
  pub mod Platform;
  ```

  All three files are declared as the *same* module, so a call site writes
  `Overlay::Platform::Art(window)` on every platform and the compiler resolves
  `Platform` to whichever file this target compiled. That is what replaced the
  three `pub use MacOS::{art, dock, front, panel, scrim}` lines: there is no
  second name for a platform function, and therefore no re-export to keep in
  step. `Overlay::Layer::Trait` is the five-operation contract - `Art`, `Scrim`,
  `Panel`, `Front`, `Dock` - stated once, so a backend that forgets an operation
  does not compile.

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
  `Front` is the one place a window is put on screen, and it is deliberately not
  `WebviewWindow::show`: that forwards to tao's `set_visible(true)`, which calls
  `makeKeyAndOrderFront:` and makes the window key. Neither the art nor the
  scrim has any use for the keyboard, and taking it would pull focus out of
  whatever the user was using; `orderFrontRegardless` orders the window front
  without the key change (and still works while the app is inactive, where
  `makeKeyAndOrderFront:` does nothing).
- **`Screen`** - one monitor resolved to the window label and the logical bounds
  a webview window takes. `Of` resolves it, `Label` keeps ASCII alphanumerics
  only (Tauri rejects the rest) and turns an unnamed monitor into
  `monitor<index>`, and `Struct` is the result.
- **`Settings`** - the persisted state. `Struct` holds `Size`, `Sizes`, `Mode`,
  `Hidden` and `Dock`; `Default`, `Maximum` and `Store` are the three constants
  (radius on a fresh install, the largest radius the panel offers, the store
  file); `Radius`, `Set` and `Clear` are the per-monitor arithmetic; `Load` and
  `Save` are the store round trip with every key spelled out; `Script` is the
  `window.settings` init script; `Visible` and `Broadcast` are what every
  command pushes to the overlay windows.
- **`Tray`** - the tray icon and its click, nothing else. `Install` builds the
  icon and calls `Menu::Toggle::Fn`. No native menu is attached, so the tray
  owns no command, no label and no state.
- **`Visibility`** - a window's wanted visibility as a value instead of a `bool`.
  `Enum` is `Shown`/`Hidden`, `Of` converts the stored boolean, and `Bool`
  converts back. It serialises as a JSON boolean, so the store keys and the
  panel's payload are unchanged by the two-variant enum.
- **`Window`** - `Chrome`, the builder flags all three layers share: borderless,
  transparent, fixed-size, always-on-top, out of the taskbar, built hidden. The
  only thing that differs between the art, the scrim and the panel is the title,
  which is a parameter.

The graph in [`Knowledge.dot`](Knowledge.dot) (rendered to `Knowledge.svg`) is
the same map with the edges drawn.

---

## Startup

1. `tauri-plugin-store` opens `settings.json` under the app config dir.
2. `Settings::Load::Fn` reads the five keys, falling back per field to the
   default (`size` 23, `mode` dark, `hidden` false, `dock_hidden` false), then
   `Settings::Save::Fn` writes the result back - which is how a fresh install
   gets its defaults.
3. `Overlay::Platform::Dock(settings.Dock)` applies the saved activation policy
   before any window exists.
4. For every monitor: `Screen::Of::Fn` resolves the label and bounds,
   `Settings::Script::Fn` builds the init script
   (`window.settings = {"size":23,"mode":"dark"};`), `Window::Chrome::Fn` applies
   the shared flags, the window is built hidden, `Overlay::Platform::Art` lifts
   it, then `Settings::Visible::Fn` fronts it unless the saved state is hidden.
5. `Tray::Install::Fn` builds the icon and wires its click to `Menu::Toggle::Fn`.
   The panel and the scrims are built on the first click, not at startup, so a
   launch that never opens the panel never pays for them.

---

## Event flow

```
tray click
  -> Menu::Toggle::Fn(app, rect)
       hidden -> Menu::Place::Fn under the icon, Backdrop::Open::Fn,
                 window.show(), Overlay::Platform::Panel above them, set_focus
       visible -> Menu::Hide::Fn: Backdrop::Close::Fn, then window.hide()
  -> the panel invokes a command: state / set_size / set_mode /
     set_hidden / set_dock / reset / quit
  -> Menu::Store::Load::Fn -> mutate -> Menu::Store::Commit::Fn
       Settings::Save::Fn
       Settings::Broadcast::Fn: for every ART window emit "size", emit "mode",
         and apply visibility only where it differs from `is_visible()` - a
         window already in the wanted state is left alone, because showing it is
         not a no-op. The panel and the scrims are skipped: the panel renders
         the state the command returns, and the scrim has no state at all
       Menu::Command::State::State - the answer the panel renders

pointerdown on a scrim
  -> invoke("dismiss")
  -> Menu::Command::Dismiss::Dismiss: Menu::Hide::Fn, or Backdrop::Close::Fn
     when no panel window is left
```

There is no other way into a dismissal. No focus event, activation probe, timer
or geometry test decides one, so none of them can get it wrong: the panel closes
when a click lands on Round's own scrim, when the tray icon is clicked, or when
the app quits.

The command names are a wire contract, not a consequence of the tree: each
command leaf holds a function named after its own path (`Size::Size`) and pins
the name the webview invokes with `#[tauri::command(rename = "set_size")]`. The
rename is what lets the function be PascalCase and the wire name stay snake_case.

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
- The `State` a command returns is
  `{ radius, maximum, ink, hidden, dock_hidden, monitors: [{ label, name, width,
  height, radius, ink, hidden }] }`, where each per-monitor setting is
  `{ value, origin }` and `origin` is `"inherit"` or `"override"`. `width` and
  `height` are the display's physical pixels, and they travel for one reason: a
  name does not identify a display. Every one of those keys is pinned with a
  per-field `#[serde(rename = "...")]`: the fields are PascalCase and the keys
  are not, so deriving one from the other is the one thing the struct cannot do.
- The panel is one scope selector and one row per setting. The monitor is the
  navigation axis, not a row: a row per monitor would be three settings times one
  default plus every monitor, so the panel draws three setting rows plus the dock
  toggle and Quit however many monitors are attached. The selector is a native
  `<select>` - the OS draws and scrolls its menu, and no entry is rendered until
  it is opened - and every row below it writes for whichever scope it names.
  `reset` with a label drops that monitor's override; with no label it restores
  the default and drops every override, which is what `Per::Set` does to the
  default too.
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

- **A new control** - a leaf under `Menu/Command/` holding the function (named
  after the leaf, with `#[tauri::command(rename = "<wire name>")]` pinning what
  the webview invokes) that mutates `Settings` and returns the
  `Menu::State::Struct` the panel should render, its entry in the
  `invoke_handler` in `Main.rs`, and the row in `Package/Web/Source/Menu.tsx`
  that calls it. Add the field to `Menu::State::Struct` - with its
  `#[serde(rename = "...")]` - when the panel has to display it.
- **A new persisted setting** - a field on `Settings::Struct`, its key in
  `Settings::Load::Fn`/`Settings::Save::Fn`, and (if the webview needs it) a
  field in `window.settings` plus the matching TypeScript type in
  `Package/Web/Source/Type/Settings.ts`.
- **A platform behaviour** - only the matching file under
  `Source/Overlay/Platform/`; it implements `Overlay::Layer::Trait`, so a missing
  operation fails to compile and no call site changes.
- **The order the three layers sit in** - the const block at the top of each
  platform file: the AppKit levels on macOS, the topmost insert on Windows, the
  raise order on X11. `Art`, `Scrim` and `Panel` are the only places a level is
  chosen.
- **A window's chrome** - `Window::Chrome::Fn`, which all three layers go
  through. A flag added there reaches the art, the scrim and the panel at once.
- **A corner's look** - `Package/Web/Source/Asset/CSS/Corner.css` and the
  `.Window` markup in `Source/Index.tsx`; the Rust side only carries numbers.
