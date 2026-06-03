#![cfg_attr(all(not(debug_assertions), target_os = "windows"), windows_subsystem = "windows")]

mod input;
mod commands;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![commands::list_devices, commands::start_listening])
        .run(tauri::generate_context!())
        .expect("failed to run tauri application");
}
