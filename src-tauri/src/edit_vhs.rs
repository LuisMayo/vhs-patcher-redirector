use std::{
    fs::{File, OpenOptions},
    iter::repeat,
    path::PathBuf,
};

use file_offset::FileExt;
use steamlocate::SteamDir;

fn unable_open_file() -> String {
    return "Couldn't open VHS file, make sure the game is closed and current user has write permissions".to_string();
}

fn string_to_big() -> String {
    return "String was too big!".to_string();
}

pub fn make_all_backups() -> Result<String, String> {
    let path = get_steamdir()?;
    move_backup(
        &path.join("Game/Binaries/Win64/Game-Win64-Shipping.exe"),
        false,
    )?;
    move_backup(
        &path.join("Game/Binaries/Win64/RedpointEOS/EOSSDK-Win64-Shipping.dll"),
        false,
    )?;
    move_backup(&path.join("VideoHorrorSociety.exe"), false)?;
    Ok("Backups made".to_string())
}

fn restore_all_backups() -> Result<String, String> {
    let path = get_steamdir()?;
    restore_backup(&path.join("Game/Binaries/Win64/Game-Win64-Shipping.exe"))?;
    restore_backup(&path.join("Game/Binaries/Win64/RedpointEOS/EOSSDK-Win64-Shipping.dll"))?;
    restore_backup(&path.join("VideoHorrorSociety.exe"))?;
    #[cfg(target_os = "windows")]
    remove_hosts_file_edit().or(Err("Error while reverting hosts changes"))?;
    #[cfg(target_os = "windows")]
    remove_certificate(&path)?;
    Ok("Backups restored".to_string())
}

#[cfg(target_os = "linux")]
fn process_vhs_file(path: &PathBuf, address: &str) -> Result<(), Box<dyn std::error::Error>> {
    let file_path = path.join("Game/Binaries/Win64/Game-Win64-Shipping.exe");
    if let Err(value) = move_backup(&file_path, false) {
        return Err(value.into());
    }
    let file_result = OpenOptions::new().write(true).open(&file_path);
    match file_result {
        Ok(file) => return write_file(file, address),
        Err(_) => return Err(unable_open_file().into()),
    }
}

#[cfg(target_os = "windows")]
fn process_vhs_file(file_path: &PathBuf, address: &str) -> Result<(), Box<dyn std::error::Error>> {
    modify_hosts_file(address)?;
    add_certificate(file_path)?;
    Ok(())
}

#[cfg(target_os = "windows")]
fn modify_hosts_file(address: &str) -> Result<String, Box<dyn std::error::Error>> {
    use std::{io::Write, path::Path};

    remove_hosts_file_edit()?;
    let path = Path::new("C:/Windows/System32/drivers/etc/hosts");
    let mut file = OpenOptions::new().append(true).open(path)?;
    let string = format!("{ip} api.vhsgame.com\r\n{ip} ns.vhsgame.com\r\n{ip} cdn.vhsgame.com\r\n{ip} mms.vhsgame.com\r\n",
        ip = address
    );
    file.write_all(string.as_bytes())?;
    return Ok("Hosts file edited".to_string());
}

#[cfg(target_os = "windows")]
fn remove_hosts_file_edit() -> Result<String, Box<dyn std::error::Error>> {
    use std::{
        fs,
        io::{Read, Write},
        path::Path,
    };

    let path = Path::new("C:/Windows/System32/drivers/etc");
    if path.try_exists()? {
        let full_path = path.join("hosts");
        let mut file = OpenOptions::new().read(true).open(&full_path)?;
        let mut buf = String::new();
        file.read_to_string(&mut buf)?;

        buf = buf.replace("\r", "");
        let new_lines: Vec<&str> = buf
            .split("\n")
            .filter(|line| !line.contains("vhsgame.com"))
            .collect();

        let new_file_content = new_lines.join("\r\n");
        fs::write(&full_path, new_file_content.as_bytes())?;
        return Ok("Removed hosts file".to_string());
    }
    Ok("Hosts file didn't exist".to_string())
}

#[cfg(target_os = "windows")]
fn add_certificate(file_path: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    use std::fs;

    use crate::RESOURCE_PATH;

    let path = file_path.join("Game/Content/Certificates");
    fs::create_dir_all(&path)?;
    let resource_lock = RESOURCE_PATH.read()?;
    match &resource_lock.certificate {
        Some(cert_path) => {
            fs::copy(cert_path, path.join("cacert.pem"))?;
            return Ok(());
        }
        None => return Err("Cannot find the certificate".into()),
    }
}

#[cfg(target_os = "windows")]
fn remove_certificate(file_path: &PathBuf) -> Result<(), String> {
    use std::io::ErrorKind;

    match std::fs::remove_file(file_path.join("Game/Content/Certificates/cacert.pem")) {
        Ok(_) => return Ok(()),
        Err(e) => {
            if e.kind() == ErrorKind::NotFound {
                return Ok(());
            } else {
                return Err("Fail to delete cer".to_string());
            }
        }
    };
}

/// returns the backup path, if any
fn move_backup(file_path: &PathBuf, restore: bool) -> Result<PathBuf, String> {
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
                        Err(_) => return Err("Error restoring backup".to_string()),
                    }
                } else {
                    return Err("Backup not found".to_string());
                }
            } else if !exists {
                let copy_result = std::fs::copy(file_path, &backup_path);
                match copy_result {
                    Ok(_) => return Ok(backup_path),
                    Err(_) => return Err("Error making/restoring backup".to_string()),
                }
            }
            return Ok(backup_path);
        }
        Err(_) => return Err("Error locating backup".to_string()),
    }
}

fn write_file(file: File, address: &str) -> Result<(), String> {
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

fn get_steamdir() -> Result<PathBuf, String> {
    let steamdirs = SteamDir::locate_multiple().unwrap_or_default();
    if steamdirs.len() == 0 {
        return Err(
            "Couldn't find Steam Location. Steam must be installed for this to work".to_string(),
        );
    } else {
        let mut found_dir: Option<PathBuf> = None;
        for dir in steamdirs {
            let result = dir.find_app(611360);
            if let Ok(option) = result {
                if let Some(path) = option {
                    found_dir = Some(path.1.resolve_app_dir(&path.0));
                    break;
                }
            }
        }
        match found_dir {
            Some(path) => return Ok(path),
            None => return Err("Couldn't find install path, is VHS installed?".to_string()),
        }
    }
}

#[tauri::command]
pub fn edit_vhs_file(address: &str) -> Result<String, String> {
    println!("Hello, world! {}", address);
    match get_steamdir() {
        Ok(app) => match process_vhs_file(&app, address) {
            Ok(_) => return Ok("Game patched".to_string()),
            Err(err) => return Err(err.to_string()),
        },
        Err(err) => return Err(err),
    }
}

fn restore_backup(path: &PathBuf) -> Result<(), String> {
    let backup_path = move_backup(&path, true)?;
    match std::fs::remove_file(backup_path) {
        Ok(_) => return Ok(()),
        Err(_) => return Err("Backup restored. Failed at removing it".to_string()),
    };
}

#[tauri::command]
pub fn restore_backup_handler() -> Result<String, String> {
    restore_all_backups()
}
