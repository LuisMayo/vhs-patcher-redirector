// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod edit_vhs;


fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![edit_vhs::edit_vhs_file])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
