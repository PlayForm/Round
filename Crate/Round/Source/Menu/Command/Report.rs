//! Debug: what one overlay webview actually holds, reported back to the trace.
//!
//! Debug build only - the module is not declared in a release build, so the
//! command does not exist there and the overlay's call cannot reach anything.

/// Record the ink and radius one overlay is rendering, as its own DOM states
/// them.
#[tauri::command]
pub fn Report(Label:String, Mode:String, Radius:String) {
	crate::Trace!("webview label={Label} data-mode={Mode} corner={Radius}");
}
