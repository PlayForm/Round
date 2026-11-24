//! Debug: what one overlay webview actually holds, reported back to the trace.

use crate::Trace;

/// Record the ink and radius one overlay is rendering, as its own DOM states
/// them. Gated by `ROUND_TRACE` inside [`Trace::Fn`].
#[tauri::command]
pub fn Report(Label:String, Mode:String, Radius:String) {
	Trace::Fn(&format!("webview label={Label} data-mode={Mode} corner={Radius}"));
}
