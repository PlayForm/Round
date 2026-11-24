//! Debug: driving a live webview, the macOS answer to CDP.
//!
//! On Windows a Tauri app can expose a real CDP endpoint by passing WebView2
//! `--remote-debugging-port`; macOS runs WKWebView, which speaks no CDP at all,
//! so there is no port to attach to. What is left is the webview's own evaluator,
//! and that is enough: this evaluates a script inside a window and returns its
//! value, which is the one capability CDP was wanted for - drive the UI, read what
//! it became.
//!
//! Debug build only: the module is not declared in a release build.

use std::time::Duration;

use tauri::{AppHandle, Manager, Runtime};

use crate::Trace;

/// Evaluate `Script` in `Window` and record its answer in the trace.
///
/// The script's last expression is the answer, awaited if it is a promise, and
/// JSON-serialised. `window.__TAURI_INTERNALS__.invoke` is the same bridge the
/// panel uses, so a page that can call a command can be driven this way.
pub fn Run<R:Runtime>(App:&AppHandle<R>, Window:&str, Script:&str) -> Result<(), String> {
	let Webview = App
		.get_webview_window(Window)
		.ok_or_else(|| format!("no window named {Window}"))?;

	let Wrapped = format!(
		r#"(async () => {{
			let Out;
			try {{
				Out = JSON.stringify(await (async () => {{ {Script} }})() ?? null);
			}} catch (Failure) {{
				Out = "ERROR: " + (Failure && Failure.message ? Failure.message : String(Failure));
			}}
			await window.__TAURI_INTERNALS__.invoke("answer", {{ window: {Window:?}, value: Out }});
		}})()"#
	);

	Webview.eval(Wrapped).map_err(|Failure| Failure.to_string())
}

/// The answer coming back from [`Run`], recorded where the trace already goes.
#[tauri::command]
pub fn Answer(Window:String, Value:String) {
	Trace!("dev {Window} -> {Value}");
}

/// Watch for a script dropped at `dev-inbox.js` and run it in the panel.
///
/// A file is the external entry point: it is the one channel a shell has into a
/// running GUI process without a debugging port, a socket, or a protocol of our
/// own. The file is consumed, so a script runs once.
pub fn Watch<R:Runtime>(App:AppHandle<R>) {
	std::thread::spawn(move || {
		let Some(Home) = std::env::var_os("HOME") else {
			return;
		};

		let Dir = std::path::Path::new(&Home).join("Library/Application Support/app.roundedcorners.binary");

		let Inbox = Dir.join("dev-inbox.js");

		loop {
			std::thread::sleep(Duration::from_millis(250));

			let Ok(Script) = std::fs::read_to_string(&Inbox) else {
				continue;
			};

			if Script.trim().is_empty() {
				continue;
			}

			// Consumed before it runs, so a script that breaks the page cannot run
			// again on the next tick.
			let _ = std::fs::remove_file(&Inbox);

			// Either a bare script (run in the panel) or `{"window":...,"script":...}`,
			// which is how an overlay is driven while the panel is closed.
			let (Target, Body) = match serde_json::from_str::<serde_json::Value>(&Script) {
				Ok(Value) => match (
					Value.get("window").and_then(serde_json::Value::as_str),
					Value.get("script").and_then(serde_json::Value::as_str),
				) {
					(Some(Window), Some(Body)) => (Window.to_owned(), Body.to_owned()),
					_ => ("menu".to_owned(), Script.clone()),
				},
				Err(_) => ("menu".to_owned(), Script.clone()),
			};

			let _ = Run(&App, &Target, &Body).inspect_err(|Failure| Trace!("dev dropped: {Failure}"));
		}
	});
}
