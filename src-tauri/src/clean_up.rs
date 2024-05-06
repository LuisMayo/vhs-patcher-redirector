pub fn clean_old_methods() -> Result<String, String> {
    match internal_clean() {
        Ok(msg) => Ok(msg),
        Err(err) => Err(err.to_string()),
    }
}

#[cfg(target_os = "windows")]
fn internal_clean() -> Result<(), Box<dyn std::error::Error>> {
    Command::new("certutil")
        .args(["-delstore", "Root", "454e1e3b62e6c326"])
        .spawn()?
        .wait()?;
    Ok("Windows detected, removed all certificate (if existing)")
}

#[cfg(not(target_os = "windows"))]
fn internal_clean() -> Result<String, Box<dyn std::error::Error>> {
    return Ok("Linux OS detected, no cleaning needed".to_string());
}
