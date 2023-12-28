use std::io::Write;

pub fn disable_core_dumps() -> Result<(), std::io::Error> {
    rlimit::setrlimit(
        rlimit::Resource::CORE, 
        0, 
        0
    )?;

    Ok(())
}

/// Disable IPv6 - Note: needs to be replaced with a non-libc implementation later.
/// 
/// # Purpose
/// This function is designed to enhance security by disabling IPv6 functionality
/// on a Unix-based system. 
/// 
/// # How it works
/// This is done by creating a socket handle for IPv6, setting the socket options to be disabled, and then closing the socket.
/// 
/// 
/// 
pub fn disable_ipv6_socket() -> Result<(), std::io::Error> {
    let mut ipv6_disabled = 1;

    // Creating ipv6 socket handle
    let ipv6_socket = unsafe { 
        libc::socket(libc::AF_INET6, libc::SOCK_DGRAM, 0) 
    };

    // Checking if ipv6 socket handle is valid
    if ipv6_socket < 0 {
        eprintln!("Unable to create socket");
    }

    // Setting socket options
    if unsafe { libc::setsockopt(
            ipv6_socket, 
            libc::IPPROTO_IPV6, 
            libc::IPV6_V6ONLY, 
            &mut ipv6_disabled as *mut _ as *mut libc::c_void, 
            std::mem::size_of::<libc::c_int>() as u32
        ) 
    } < 0 {
        eprintln!("Unable to set socket options");
    }

    // Closing ipv6 socket
    unsafe { 
        libc::close(ipv6_socket) 
    };

    Ok(())
}

fn disable_ipv6_conf() -> Result<(), std::io::Error> {
    // Opening /proc/sys/net/ipv6/conf/all/disable_ipv6
    let fp = std::fs::OpenOptions::new()
        .write(true) // Open file in write mode
        .open("/proc/sys/net/ipv6/conf/all/disable_ipv6")? // If unable to open file, print error message
        .write_all(b"1")?; // Write 1 to /proc/sys/net/ipv6/conf/all/disable_ipv6

    Ok(())
}

fn disable_setuid_binaries() -> Result<(), std::fmt::Error> {
    // Disable the ability to use setuid binaries
    prctl::set_no_new_privileges(true).expect("Unable to disable setuid binaries");
    Ok(())
}

/// Disable ptrace
/// 
/// # From Original C:
/// ```
/// void disable_ptrace() {
///    if (prctl(PR_SET_DUMPABLE, 0) == -1) {
///        perror("prctl");
///    }
/// }
/// ```
fn disable_ptrace() -> Result<(), std::fmt::Error> {
    // Disable ptrace
    prctl::set_dumpable(false).expect("Unable to disable ptrace");
    Ok(())
}

/// Disable loading of kernel modules
/// 
/// # From Original C:
/// ```
/// void disable_loading_kernel_modules()
/// {
///     // Disable loading kernel modules
///     int fd = open("/proc/sys/kernel/modules_disabled", O_WRONLY);
///     if (fd == -1) {
///         perror("open");
///     } else {
///         if (write(fd, "1", 1) == -1) {
///             perror("write");
///         }
///         close(fd);
///     }
/// }
/// ```
fn disable_kernel_modules() -> Result<(), std::io::Error> {
    // Disable loading of kernel modules
    let fd = std::fs::OpenOptions::new()
        .write(true) // Open file in write mode
        .open("/proc/sys/kernel/modules_disabled")? // Open file by path
        .write_all(b"1")?; // Write 1 to file

    Ok(())
}

fn disable_loading_usb_storage() -> Result<(), std::io::Error> {
    let fd = std::fs::OpenOptions::new()
        .write(true) // Open file in write mode
        .open("/etc/modprobe.d/blacklist-usb-storage.conf")? // Open file by path
        .write_all(b"blacklist usb-storage\n")?; // Write 1 to file

    Ok(())
}
///
/// # From Original C:
/// ```
/// void enable_aslr() {
///     struct rlimit rl;
///     if (getrlimit(RLIMIT_STACK, &rl) == 0) {
///         rl.rlim_cur = RLIM_INFINITY;
///         if (setrlimit(RLIMIT_STACK, &rl) != 0) {
///         }
///     }
/// }
/// ```
fn enable_aslr() -> Result<(), std::io::Error> {
    if rlimit::getrlimit(rlimit::Resource::STACK)
    .unwrap() == (0, 0) {
        rlimit::setrlimit(
            rlimit::Resource::STACK, 
            rlimit::INFINITY, 
            rlimit::INFINITY
        )?;
    }
    Ok(())
}

///
/// # Original C:
/// ```
/// void harden_sshd() {
///     // Open the sshd_config file for writing
///    FILE *file = fopen("/etc/ssh/sshd_config", "w");
///    if (file == NULL) {
///        perror("Error opening file");
///        return 1;
///    }
///
///    // Write the contents of the CIS Benchmark Compliant sshd_config file to sshd_config
///    fprintf(file, "# CIS Benchmark Compliant sshd_config\n");
///    fprintf(file, "# Set SSH Protocol Version 2\n");
///    fprintf(file, "Protocol 2\n");
///    fprintf(file, "# Disable X11 Forwarding\n");
///    fprintf(file, "X11Forwarding no\n");
///    fprintf(file, "# Set the maximum authentication retries to 3\n");
///    fprintf(file, "# Set the maximum sessions to 10\n");
///    fprintf(file, "MaxSessions 10\n");
///    fprintf(file, "# Disable GSSAPI authentication\n");
///    fprintf(file, "GSSAPIAuthentication no\n");
///    fprintf(file, "# Set the MAC algorithms to the following\n");
///    fprintf(file, "MACs hmac-sha2-256,hmac-sha2-512\n");
///    fprintf(file, "# Set the allowed ciphers to the following\n");
///    fprintf(file, "Ciphers aes128-ctr,aes192-ctr,aes256-ctr\n");
///    fprintf(file, "# Set the allowed key exchange algorithms to the following\n");
///    fprintf(file, "KexAlgorithms curve25519-sha256@libssh.org,diffie-hellman-group-exchange-sha256\n");
///    fprintf(file, "ClientAliveInterval 300\n");
///    fprintf(file, "LogLevel INFO\n"); 
///    fprintf(file, "IgnoreRhosts yes\n"); 
///    fprintf(file, "HostbasedAuthentication no\n");
///    fprintf(file, "PermitEmptyPasswords no\n");
///    fprintf(file, "PermitRootLogin yes\n");
///    fprintf(file, "LoginGraceTime 60\n");
///    fprintf(file, "Subsystem sftp internal-sftpz\n");
///    fprintf(file, "UsePAM yes\n");
///
///
///    // Close the sshd_config file
///    fclose(file);
///
///    // Set the owner and group of /etc/ssh/sshd_config to root
///    chown("/etc/ssh/sshd_config", 0, 0);
///
///    // Set the file permissions of /etc/ssh/sshd_config to 600 (rw-------)
///    chmod("/etc/ssh/sshd_config", S_IRUSR | S_IWUSR);
///
///
///    restart_service("sshd");
///
/// }
/// ```
fn harden_sshd() -> std::io::Result<()> {
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

    unsafe{ 
        let path = std::ffi::CString::new("/etc/ssh/sshd_config")?;

        libc::chown(path.as_ptr(), 0, 0);
        libc::chmod(path.as_ptr(),libc::S_IRUSR | libc::S_IWUSR);
    };
    Ok(())
}