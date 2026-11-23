//! Leaving.

use tauri::AppHandle;

/// Quit the whole process.
#[tauri::command(rename = "quit")]
pub fn Quit(App:AppHandle) { App.exit(0); }
