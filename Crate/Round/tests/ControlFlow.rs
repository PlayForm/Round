//! Does a click travel the whole path?
//!
//! Each test sends a real IPC request into the handler the binary registers -
//! `Handler::Fn()` - against a mock runtime, then reads the store the command
//! wrote. That covers command -> load -> write -> save -> broadcast -> state,
//! which no unit test of `Per::Set` can reach: the defect that started this was
//! exactly a step between those two ends.
//!
//! The mock runtime has no displays, so `monitors` comes back empty. That is the
//! point: a monitor's override is stored by label, and the label does not have to
//! resolve to a live display for the write to be worth testing.

// House style: locals and parameters are PascalCase, the same as the library's.
#![allow(non_snake_case)]

use serde_json::{Value, json};
use tauri::{
	Manager,
	WebviewUrl,
	WebviewWindowBuilder,
	test::{INVOKE_KEY, MockRuntime, get_ipc_response, mock_builder, mock_context, noop_assets},
	webview::InvokeRequest,
};
use tauri_plugin_store::StoreExt;

const STORE:&str = "settings.json";

/// The surface this build registers - the same split the binary makes, so a
/// `--release` test run exercises the shipping list rather than a debug one.
#[cfg(debug_assertions)]
fn Handler() -> impl Fn(tauri::ipc::Invoke<MockRuntime>) -> bool + Send + Sync + 'static { pround::Commands!() }

#[cfg(not(debug_assertions))]
fn Handler() -> impl Fn(tauri::ipc::Invoke<MockRuntime>) -> bool + Send + Sync + 'static { pround::CommandsRelease!() }

fn App() -> (tauri::App<MockRuntime>, tauri::WebviewWindow<MockRuntime>) {
	let App = mock_builder()
		.plugin(tauri_plugin_store::Builder::new().build())
		.invoke_handler(Handler())
		.build(mock_context(noop_assets()))
		.expect("a mock app");

	// The set a launch resolves, seeded by hand: a mock runtime has no displays,
	// and the command layer reads exactly this.
	App.manage(pround::Screen::Registry::Struct(vec![
		Screen("Monitor26344", "Monitor #26344", 3328, 1872),
		Screen("Monitor22763", "Monitor #22763", 1800, 3200),
	]));

	let Window = WebviewWindowBuilder::new(&App, "menu", WebviewUrl::default())
		.build()
		.expect("the panel window");

	(App, Window)
}

/// One resolved monitor, as a launch would hand it over.
fn Screen(Label:&str, Name:&str, Width:i32, Height:i32) -> pround::Screen::Struct::Struct {
	pround::Screen::Struct::Struct {
		Label:Label.into(),
		Qualified:Label.into(),
		Legacy:Label.into(),
		Name:Name.into(),
		Pixels:(Width, Height),
		Width,
		Height,
		X:0,
		Y:0,
	}
}

/// One command, through IPC, answered.
fn Call(Window:&tauri::WebviewWindow<MockRuntime>, Command:&str, Body:Value) -> Value {
	get_ipc_response(
		Window,
		InvokeRequest {
			cmd:Command.into(),
			callback:tauri::ipc::CallbackFn(0),
			error:tauri::ipc::CallbackFn(1),
			url:"tauri://localhost".parse().expect("a url"),
			body:Body.into(),
			headers:Default::default(),
			invoke_key:INVOKE_KEY.to_string(),
		},
	)
	.expect("the command answered")
	.deserialize::<Value>()
	.expect("the answer is json")
}

fn Stored(App:&tauri::App<MockRuntime>, Key:&str) -> Value {
	App.store(STORE)
		.expect("the store")
		.get(Key)
		.unwrap_or_else(|| panic!("{Key} was never written"))
}

#[test]
fn one_monitor_holds_its_own_radius() {
	let (App, Window) = App();

	Call(&Window, "set_size", json!({ "label":"Monitor26344", "value":37 }));

	let Radius = Stored(&App, "radius");

	assert_eq!(Radius["monitor"]["Monitor26344"], json!(37));
	assert_ne!(Radius["default"], json!(37), "a monitor's value must not become the default");
}

#[test]
fn setting_the_default_drops_every_override() {
	let (App, Window) = App();

	Call(&Window, "set_size", json!({ "label":"Monitor26344", "value":37 }));

	Call(&Window, "set_size", json!({ "value":23 }));

	let Radius = Stored(&App, "radius");

	assert_eq!(Radius["default"], json!(23));
	assert_eq!(Radius["monitor"], json!({}), "the default is every monitor's value");
}

#[test]
fn ink_is_per_monitor() {
	let (App, Window) = App();

	Call(&Window, "set_mode", json!({ "label":"Monitor26344", "mode":"light" }));
	Call(&Window, "set_mode", json!({ "label":"Monitor22763", "mode":"dark" }));

	let Ink = Stored(&App, "ink");

	assert_eq!(Ink["monitor"]["Monitor26344"], json!("light"));
	assert_eq!(Ink["monitor"]["Monitor22763"], json!("dark"));
}

#[test]
fn reset_drops_only_the_named_monitor() {
	let (App, Window) = App();

	Call(&Window, "set_size", json!({ "label":"Monitor26344", "value":37 }));
	Call(&Window, "set_size", json!({ "label":"Monitor22763", "value":41 }));

	Call(&Window, "reset", json!({ "setting":"radius", "label":"Monitor26344" }));

	let Radius = Stored(&App, "radius");

	assert!(
		Radius["monitor"].get("Monitor26344").is_none(),
		"the reset monitor follows the default again"
	);
	assert_eq!(
		Radius["monitor"]["Monitor22763"],
		json!(41),
		"the other monitor keeps its own value"
	);
}

#[test]
fn the_answer_carries_every_monitor_with_its_origin() {
	let (_App, Window) = App();

	let Answer = Call(&Window, "state", json!({}));

	assert!(Answer["maximum"].is_number(), "the slider's range travels with the state");
	assert!(
		Answer["monitors"].as_array().is_some_and(|Each| Each.len() == 2),
		"one entry per monitor"
	);
}
