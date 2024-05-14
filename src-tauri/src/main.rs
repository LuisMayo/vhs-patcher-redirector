// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{path::PathBuf, sync::RwLock};

use admin_check::admin_check;
use edit_vhs::make_all_backups;
use replies::{GameMsg, InitResponse};

mod admin_check;
mod clean_up;
mod edit_vhs;
mod my_error;
mod replies;

#[derive(Default)]
struct ResourcePaths {
    pub certificate: Option<PathBuf>,
    pub eos: Option<PathBuf>,
    pub exe: Option<PathBuf>,
}

static RESOURCE_PATH: RwLock<ResourcePaths> = RwLock::new(ResourcePaths {
    eos: None,
    certificate: None,
    exe: None,
});

#[tauri::command]
fn init() -> InitResponse {
    let results = [
        clean_up::clean_old_methods(),
        make_all_backups(),
        admin_check(),
    ];
    let map: Vec<GameMsg> = results
        .into_iter()
        .map(|result| match result {
            Ok(msg) => GameMsg { success: true, msg },
            Err(err) => GameMsg {
                success: false,
                msg: err,
            },
        })
        .collect();
    #[cfg(target_os = "windows")]
    let platform = "windows".to_string();
    #[cfg(not(target_os = "windows"))]
    let platform = "linux".to_string();
    return InitResponse {
        msgs: map,
        platform: platform,
    };
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            edit_vhs::edit_vhs_file,
            edit_vhs::restore_backup_handler,
            edit_vhs::edit_vhs_and_add_mod,
            init
        ])
        .setup(|app| {
            let exe = app
                .path_resolver()
                .resolve_resource("resources/VideoHorrorSociety.exe");
            let cert = app.path_resolver().resolve_resource("resources/cacert.pem");
            let eos = app
                .path_resolver()
                .resolve_resource("resources/EOSSDK-Win64-Shipping.dll");
            let mut resource_lock = RESOURCE_PATH.write()?;
            resource_lock.certificate = cert;
            resource_lock.exe = exe;
            resource_lock.eos = eos;
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
