use std::process::{Command, Stdio};

pub fn disable_source_routing() -> Result<(), std::io::Error> {
    // Spawn a child process to execute sysctl
    let output = Command::new("/sbin/sysctl")
        .arg("-w")
        .arg("net.ipv4.conf.all.accept_source_route=0")
        .arg("net.ipv4.conf.default.accept_source_route=0")
        .stdout(Stdio::piped())
        .spawn()?
        .wait_with_output()?;

    if !output.status.success() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("Error executing sysctl: {}", String::from_utf8_lossy(&output.stderr)),
        ));
    }

    Ok(())
}
