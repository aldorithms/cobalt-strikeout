use std::process::{Stdio, Command};

pub fn ensure_sudo_log_file_exists() -> Result<(), std::io::Error> {
    // Spawn a child process to execute sysctl
    let _ = Command::new("/sbin/sysctl")
        .arg("-w")
        .arg("Defaults log_file")
        .stdout(Stdio::piped())
        .spawn()?
        .wait_with_output()?;

    Ok(())
}