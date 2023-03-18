<p align="center">
  <img width="64" height="64" src="./src-tauri/src/img/icon.ico">
</p>

# Rounded Corners

<a href ="https://discord.gg/RzjXMMjY27" target="_blank">
	<picture>
		<source
			media="(prefers-color-scheme: dark)"
			srcset={`https://img.shields.io/discord/977956954041356329?label=discord&logo=discord&color=black&logoColor=white&labelColor=black&logoWidth=15`}
		/>
		<source
			media="(prefers-color-scheme: light)"
			srcset={`https://img.shields.io/discord/977956954041356329?label=discord&logo=discord&color=white&logoColor=black&labelColor=white&logoWidth=15`}
		/>
		<img
			alt="Discord"
			src={`https://img.shields.io/discord/977956954041356329?label=discord&logo=discord&color=black&logoColor=white&labelColor=black&logoWidth=15`}
		/>
	</picture>
</a>

https://discord.gg/tnsnMpbK This app rounds the corners of your Windows screen.

![Rounded Corners](./.github/cover.png)

## Getting started

Rounded Corners sets up a system tray application using the Tauri framework. It
creates a window for each monitor available on the system, sets up a menu for
the system tray, and handles events from the menu.

## Code

The code imports several crates:

-   `regex`
-   `serde_json`
-   `tauri`
-   `tauri_plugin_store`

`regex` provides support for regular expressions, `serde_json` is a JSON
serialization/deserialization library, `tauri` is the main framework for
building cross-platform desktop apps in Rust, and `tauri_plugin_store` provides
a key-value store for persisting application data.

## Options

The app has several menu items:

-   ➕ **Increase Size**
-   ➖ **Decrease Size**
-   ↩️ **Reset**
-   🌑 **Dark**
-   ☀️ **Light**
-   👨🏻 **Show**
-   🥷🏽 **Hide**
-   ❌ **Exit**

Clicking on **Increase Size** or **Decrease Size** increases or decreases the
roundness of the windows, respectively.

**Reset** sets the corner radius back to the default value of _23px_.

**Dark** and **Light** switch the app between dark and light mode.

**Show** and **Hide** show or hide all windows, respectively.

**Exit** closes the app.

## Changelog

See [CHANGELOG.md](CHANGELOG.md) for a history of changes to this product.

[![Lightrix logo](https://raw.githubusercontent.com/Lightrix/npm/main/.github/img/favicon.png "Built with Lightrix/npm")](https://github.com/Lightrix/npm)
