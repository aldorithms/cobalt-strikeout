use std::process::{Command, Stdio};


/// # Description
/// sudo can be configured to run only from a pseudo-pty
/// # Rationale
/// Attackers can run a malicious program using sudo,
/// which would again fork a background process that remains even when the main program has finished executing
pub fn enable_tcp_syn_cookies() -> Result<(), std::io::Error> {
    // Spawn a child process to execute sysctl
    let output = Command::new("/sbin/sysctl")
        .arg("-w")
        .arg("net.ipv4.tcp_syncookies=1")
        .stdout(Stdio::piped())
        .spawn()?
        .wait_with_output()?;

    if !output.status.success() {
        return Err(
            std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Error executing sysctl: {}", String::from_utf8_lossy(&output.stderr)),
            )
        );
    }

    Ok(())
}
