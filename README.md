<table><tr> <td colspan="1"> <h3 align="center"> <picture> <source media="(prefers-color-scheme: dark)" srcset="https://playform.cloud/Image/GitHub/Round/32x32.png"> <source media="(prefers-color-scheme: light)" srcset="https://playform.cloud/Image/GitHub/Round/32x32.png"> <img width="28" alt="" src="https://playform.cloud/Image/GitHub/Round/32x32.png"> </picture>  </h3> </td> <td colspan="3" valign="top"> <h3 align="center"> Round — </h3> </td> </tr><tr><td valign="top" colspan="2"><a href="HTTPS://GitHub.Com/PlayForm/Round" target="_blank"> <picture> <source media="(prefers-color-scheme: dark)" srcset="https://img.shields.io/github/last-commit/PlayForm/Round?label=Update&color=black&labelColor=black&logoColor=white&logoWidth=0"> <source media="(prefers-color-scheme: light)" srcset="https://img.shields.io/github/last-commit/PlayForm/Round?label=Update&color=white&labelColor=white&logoColor=black&logoWidth=0"> <img src="https://img.shields.io/github/last-commit/PlayForm/Round?label=Update&color=black&labelColor=black&logoColor=white&logoWidth=0" alt="Update" title="Update"> </picture> </a><br><a href="HTTPS://GitHub.Com/PlayForm/Round" target="_blank"> <picture> <source media="(prefers-color-scheme: dark)" srcset="https://img.shields.io/github/issues/PlayForm/Round?label=Issue&color=black&labelColor=black&logoColor=white&logoWidth=0"> <source media="(prefers-color-scheme: light)" srcset="https://img.shields.io/github/issues/PlayForm/Round?label=Issue&color=white&labelColor=white&logoColor=black&logoWidth=0"> <img src="https://img.shields.io/github/issues/PlayForm/Round?label=Issue&color=black&labelColor=black&logoColor=white&logoWidth=0" alt="Issue" title="Issue"> </picture> </a><br></td><td valign="top" colspan="2"><a href="https://github.com/PlayForm/Round" target="_blank"><picture><source media="(prefers-color-scheme: dark)" srcset="https://img.shields.io/github/stars/PlayForm/Round?style=flat&label=Star&logo=github&color=black&labelColor=black&logoColor=white&logoWidth=0"><source media="(prefers-color-scheme: light)" srcset="https://img.shields.io/github/stars/PlayForm/Round?style=flat&label=Star&logo=github&color=white&labelColor=white&logoColor=black&logoWidth=0"><img src="https://img.shields.io/github/stars/PlayForm/Round?style=flat&label=Star&logo=github&color=black&labelColor=black&logoColor=white&logoWidth=0" alt="Star"></picture></a><br><a href="HTTPS://GitHub.Com/PlayForm/Round" target="_blank"> <picture> <source media="(prefers-color-scheme: dark)" srcset="https://img.shields.io/github/downloads/PlayForm/Round/total?label=Download&color=black&labelColor=black&logoColor=white&logoWidth=0"> <source media="(prefers-color-scheme: light)" srcset="https://img.shields.io/github/downloads/PlayForm/Round/total?label=Download&color=white&labelColor=white&logoColor=black&logoWidth=0"> <img src="https://img.shields.io/github/downloads/PlayForm/Round/total?label=Download&color=black&labelColor=black&logoColor=white&logoWidth=0" alt="Download" title="Download"> </picture> </a><br><a href="https://github.com/PlayForm/Round" target="_blank"><b>⬜ Round —</b></a></td></tr></table><p align="center">
  <img width="64" height="64" src="https://playform.cloud/Image/GitHub/Round/icon.ico" alt="Round" />
</p>

# Round

Rounds the corners of your Windows screen.

![Round](https://playform.cloud/Image/GitHub/Round/Cover.png)

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
