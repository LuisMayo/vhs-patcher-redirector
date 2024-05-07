// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use admin_check::admin_check;
use edit_vhs::make_all_backups;
use replies::GameMsg;

mod edit_vhs;
mod clean_up;
mod replies;
mod admin_check;
mod my_error;


#[tauri::command]
fn init() -> Vec<GameMsg>  {
    let results = [clean_up::clean_old_methods(), make_all_backups(), admin_check()];
    let map: Vec<GameMsg> = results.into_iter().map(|result| {
        match result {
            Ok(msg) =>GameMsg{success: true, msg },
            Err (err) => GameMsg {success: false, msg: err}
        }
    }).collect();
    return map;
}



fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![edit_vhs::edit_vhs_file, edit_vhs::restore_backup_handler, init])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
