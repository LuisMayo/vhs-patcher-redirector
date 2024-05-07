pub fn clean_old_methods() -> Result<String, String> {
    match internal_clean() {
        Ok(msg) => Ok(msg),
        Err(err) => Err(err.to_string()),
    }
}

#[cfg(target_os = "windows")]
fn internal_clean() -> Result<String, Box<dyn std::error::Error>> {
    use std::{io::Read, process::Stdio};
    use std::process::Command;

    use crate::my_error::MyError;

    let mut command = Command::new("certutil");
    command.args(["-delstore", "Root", "454e1e3b62e6c326"])
    .stdout(Stdio::piped());


    let mut child = command.spawn()?;
    let code = child.wait()?;
    if code.success() {
        Ok("Windows detected, removed all certificate (if existing)".to_string())
    } else {
        if let Some(stdout_bytes) = child.stdout {
            let vec = stdout_bytes.bytes().map(|byte_res| byte_res.unwrap_or_default()).collect::<Vec<u8>>().to_vec();
            let output = std::str::from_utf8(&vec).unwrap_or_default();
            let mut err_msg = code.to_string();
            err_msg.push_str(output);
            return Err(Box::new(MyError {msg: err_msg}));
        }
        return Err(Box::new(MyError {msg: "".to_string()}));
    }
}

#[cfg(not(target_os = "windows"))]
fn internal_clean() -> Result<String, Box<dyn std::error::Error>> {
    return Ok("Linux OS detected, no cleaning needed".to_string());
}
