<p align="center">
  <img width="64" height="64" src="https://nikolahristov.tech/Image/GitHub/Round/icon.ico" alt="Round" />
</p>

# Round

Rounds the corners of your Windows screen.

![Round](https://nikolahristov.tech/Image/GitHub/Round/Cover.png)

## Getting started

`Round` sets up a system tray application using the Tauri framework. It creates
a window for each monitor available on the system, sets up a menu for the system
tray, and handles events from the menu.

## Dependencies

The code imports several crates:

-   `regex` - provides support for regular expressions
-   `serde_json`- is a JSON serialization/deserialization library
-   `tauri` - is the main framework for building cross-platform desktop apps in
    Rust
-   `tauri_plugin_store` - provides a key-value store for persisting application
    data

## Options

The app has several menu items:

-   ➕ **Increase Size**
-   ➖ **Decrease Size**
-   ↩️ **Reset**
-   🌑 **Dark**
-   ☀️ **Light**
-   👨🏻 **Show**
-   🥷🏽 **Hide**
-   ❌ **Exit**

Clicking on **Increase Size** or **Decrease Size** increases or decreases the
roundness of the windows, respectively.

**Reset** sets the corner radius back to the default value of _23px_.

**Dark** and **Light** switch the app between dark and light mode.

**Show** and **Hide** show or hide all windows, respectively.

**Exit** closes the app.

## Changelog

See [CHANGELOG.md](CHANGELOG.md) for a history of changes to this app.
