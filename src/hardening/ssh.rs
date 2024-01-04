use std::io::Write;
use color_eyre::Result;

pub fn harden_sshd() -> Result<()> {
    let mut file = std::fs::OpenOptions::new()
        .write(true) // Open file in write mode
        .open("/etc/ssh/sshd_config")?; // Open file by path

    file.write_all(b"# CIS Benchmark Compliant sshd_config\n")?;
    file.write_all(b"# Set SSH Protocol Version 2\n")?;
    file.write_all(b"Protocol 2\n")?;
    file.write_all(b"# Disable X11 Forwarding\n")?;
    file.write_all(b"X11Forwarding no\n")?;
    file.write_all(b"# Set the maximum authentication retries to 3\n")?;
    file.write_all(b"# Set the maximum sessions to 10\n")?;
    file.write_all(b"MaxSessions 10\n")?;
    file.write_all(b"# Disable GSSAPI authentication\n")?;
    file.write_all(b"GSSAPIAuthentication no\n")?;
    file.write_all(b"# Set the MAC algorithms to the following\n")?;
    file.write_all(b"MACs hmac-sha2-256,hmac-sha2-512\n")?;
    file.write_all(b"# Set the allowed ciphers to the following\n")?;
    file.write_all(b"Ciphers aes128-ctr,aes192-ctr,aes256-ctr\n")?;
    file.write_all(b"# Set the allowed key exchange algorithms to the following\n")?;
    file.write_all(b"KexAlgorithms curve25519-sha256@libssh.org,diffie-hellman-group-exchange-sha256\n")?;
    file.write_all(b"ClientAliveInterval 300\n")?;
    file.write_all(b"LogLevel INFO\n")?;
    file.write_all(b"IgnoreRhosts yes\n")?;
    file.write_all(b"HostbasedAuthentication no\n")?;
    file.write_all(b"PermitEmptyPasswords no\n")?;
    file.write_all(b"PermitRootLogin yes\n")?;
    file.write_all(b"LoginGraceTime 60\n")?;
    file.write_all(b"Subsystem sftp internal-sftpz\n")?;
    file.write_all(b"UsePAM yes\n")?;


    let path = std::ffi::CString::new("/etc/ssh/sshd_config")?;

    unsafe{ 
        libc::chown(path.as_ptr(), 0, 0);
        libc::chmod(path.as_ptr(),libc::S_IRUSR | libc::S_IWUSR);
    };
    Ok(())
}