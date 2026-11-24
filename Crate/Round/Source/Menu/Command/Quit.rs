//! Leaving.

use tauri::{AppHandle, Runtime};

/// Quit the whole process.
#[tauri::command(rename = "quit")]
pub fn Quit<R:Runtime>(App:AppHandle<R>) { App.exit(0); }
