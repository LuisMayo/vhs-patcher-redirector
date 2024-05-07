pub fn admin_check() -> Result<String, String> {
    match admin_check_internal() {
        Ok(msg) => Ok(msg),
        Err(err) => Err(err.to_string()),
    }
}

#[cfg(target_os = "windows")]
fn admin_check_internal() -> Result<String, Box<dyn std::error::Error>> {
    use std::process::{Command, Stdio};

    if Command::new("fsutil")
    .args(["dirty", "query", env!("systemdrive")])
    .stderr(Stdio::null())
    .stdout(Stdio::null())
    .spawn()?
    .wait()?.success() {
        Ok("App launched with admin rights".to_string())
    } else {
        Err("App doesn't have admin righs".into())
    }
}


#[cfg(not(target_os = "windows"))]
fn admin_check_internal() -> Result<String, String> {
    return Ok("Linux OS detected, no admin needed".to_string());
}