use std::{
    fs::{File, OpenOptions},
    iter::repeat,
    path::PathBuf,
};

use file_offset::FileExt;
use steamlocate::SteamDir;


fn unable_open_file() -> &'static str {
    return "Couldn't open VHS file, make sure the game is closed and current user has write permissions";
}

fn string_to_big() -> &'static str {
    return "String was too big!";
}

fn process_vhs_file(file_path: &PathBuf, address: &str) -> Result<(), &'static str> {
    if let Err(value) = move_backup(&file_path, false) {
        return Err(value);
    }
    let file_result = OpenOptions::new().write(true).open(&file_path);
    match file_result {
        Ok(file) => return write_file(file, address),
        Err(_) => return Err(unable_open_file()),
    }
}

/// returns the backup path, if any
fn move_backup(file_path: &PathBuf, restore: bool) -> Result<PathBuf, &'static str> {
    let mut backup_path = file_path.clone();
    backup_path.set_extension("bak");
    let exists_result = backup_path.try_exists();
    match exists_result {
        Ok(exists) => {
            if restore {
                if exists {
                    let copy_result = std::fs::copy(&backup_path, file_path);
                    match copy_result {
                        Ok(_) => return Ok(backup_path),
                        Err(_) => return Err("Error restoring backup")
                    }
                } else {
                    return Err("Backup not found");
                }
            } else if !exists {
                let copy_result = std::fs::copy(file_path, &backup_path);
                match copy_result {
                    Ok(_) => return Ok(backup_path),
                    Err(_) => return Err("Error making/restoring backup")
                }
            } 
            return Ok(backup_path);
        },
        Err(_) => return Err("Error locating backup")
    }
}

fn write_file(file: File, address: &str) -> Result<(), &'static str> {
    const BUFFER_SIZE: usize = 0x80;
    let mut buffer: Vec<u8> = address
        .encode_utf16()
        .map(|item| item.to_le_bytes())
        .flatten()
        .collect();
    if buffer.len() > BUFFER_SIZE {
        return Err(string_to_big());
    } else {
        buffer.extend(repeat(0).take(BUFFER_SIZE - buffer.len()));
        file.write_offset(&buffer, 0x5382CA0)
            .expect("Unable to write on the file");
        return Ok(());
    }
}

fn get_steamdir() -> Option<PathBuf> {
    let dir = SteamDir::locate()?.app(&611360)?.path.join("Game/Binaries/Win64/Game-Win64-Shipping.exe");
    return Some(dir);
}

#[tauri::command]
pub fn edit_vhs_file(address: &str) -> Result<(), &str> {
    println!("Hello, world! {}", address);
    match get_steamdir() {
        Some(app) => return process_vhs_file(&app, address),
        None => return Err("Unable to locate Steam or game"),
    }
}

fn restore_backup(path: &PathBuf) -> Result<(), &'static str> {
    let backup_path = move_backup(&path, true)?;
    match std::fs::remove_file(backup_path) {
        Ok(_) => return Ok(()),
        Err(_) => return Err("Backup restaurado, pero error al borrarlo")
    };
}

#[tauri::command]
pub fn restore_backup_handler() -> Result<(), &'static str> {
    match get_steamdir() {
        Some(app) => return restore_backup(&app),
        None => return Err("Unable to locate Steam or game"),
    }
}