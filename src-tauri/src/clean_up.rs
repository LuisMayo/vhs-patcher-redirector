#[cfg(target_os = "windows")]
use std::process::Command;

pub fn clean_old_methods() -> Result<String, String> {
    match internal_clean() {
        Ok(msg) => Ok(msg),
        Err(err) => Err(err.to_string()),
    }
}

#[cfg(target_os = "windows")]
fn internal_clean() -> Result<String, Box<dyn std::error::Error>> {
    use std::{io::Read, process::Stdio};

    let command =Command::new("certutil")
    .args(["-delstore", "Root", "454e1e3b62e6c326"])
    .stdout(Stdio::piped());


    let child = command.spawn()?;
    let code = child.wait()?;
    if code.success() {
        Ok("Windows detected, removed all certificate (if existing)".to_string())
    } else {
        if let Some(stdout_bytes) = child.stdout {
            let output =std::str::from_utf8(stdout_bytes.bytes().into())?;
            return Err(output)
        }
        return Err("");
    }
}

#[cfg(not(target_os = "windows"))]
fn internal_clean() -> Result<String, Box<dyn std::error::Error>> {
    return Ok("Linux OS detected, no cleaning needed".to_string());
}
