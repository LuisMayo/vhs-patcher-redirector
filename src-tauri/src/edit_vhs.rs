use std::{
    fs::{File, OpenOptions},
    iter::repeat,
    os::windows::prelude::FileExt,
    path::PathBuf,
};

use steamlocate::SteamDir;

fn unable_locate_dir() {
    println!("Couldn't locate Steam or Vhs install dir");
}

fn unable_open_file() {
    println!("Couldn't open VHS file, make sure the game is closed and current user has write permissions");
}

fn string_to_big() {
    println!("String was too big! :(");
}

fn success() {
    println!("ole ole los caracoles");
}

fn process_vhs_file(game_dir: &PathBuf, address: &str) {
    let file_path = game_dir.join("Game/Binaries/Win64/Game-Win64-Shipping.exe");
    let mut backup_path = file_path.clone();
    backup_path.set_extension("bak");
    let _ = std::fs::copy(&file_path, &backup_path);
    let file_result = OpenOptions::new().write(true).open(&file_path);
    match file_result {
        Ok(file) => write_file(file, address),
        Err(_) => unable_open_file(),
    }
}

fn write_file(file: File, address: &str) {
    const BUFFER_SIZE: usize = 0x80;
    let address = "https://apps.luismayo.com/vhs-%s/%s/%s/?guid=%s";
    let mut buffer: Vec<u8> = address
        .encode_utf16()
        .map(|item| item.to_le_bytes())
        .flatten()
        .collect();
    if buffer.len() > BUFFER_SIZE {
        string_to_big();
    } else {
        buffer.extend(repeat(0).take(BUFFER_SIZE - buffer.len()));
        file.seek_write(&buffer, 0x5382CA0)
            .expect("Unable to write on the file");
        success();
    }
}

#[tauri::command]
fn edit_vhs_file(address: &str) {
    println!("Hello, world!");
    match SteamDir::locate() {
        Some(mut steamdir) => match steamdir.app(&611360) {
            Some(app) => process_vhs_file(&app.path, address),
            None => unable_locate_dir(),
        },
        None => unable_locate_dir(),
    }
}
