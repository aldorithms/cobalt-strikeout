use std::process::{Stdio, Command};

pub fn ensure_sudo_uses_pty() -> Result<(), std::io::Error> {
    // Spawn a child process to execute sysctl
    let _ = Command::new("/sbin/sysctl")
        .arg("-w")
        .arg("Defaults use_pty")
        .stdout(Stdio::piped())
        .spawn()?
        .wait_with_output()?;

    Ok(())
}