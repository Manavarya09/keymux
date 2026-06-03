use crate::input::{enumerate_devices, start_listening, DeviceInfo, KeyEvent};
use tauri::Manager;

#[tauri::command]
pub fn list_devices() -> Result<Vec<DeviceInfo>, String> {
    enumerate_devices().map_err(|e| e.to_string())
}

#[tauri::command]
pub fn start_listening() -> Result<(), String> {
    // For Phase 1, start_listening registers a callback that currently is a no-op on non-Windows.
    start_listening(Box::new(move |_e: KeyEvent| {
        // TODO: send events through a channel or app handle in next iteration
    }))
    .map_err(|e| e.to_string())
}
