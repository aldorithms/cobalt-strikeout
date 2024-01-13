use std::{io::Write, fs::OpenOptions, mem::size_of};
use color_eyre::Result;
use libc::{IPPROTO_IPV6, IPV6_V6ONLY, c_void, c_int};


/// Disable IPv6 - Note: needs to be replaced with a non-libc implementation later.
/// 
/// # Purpose
/// This function is designed to enhance security by disabling IPv6 functionality
/// on a Unix-based system. 
/// 
/// # How it works
/// This is done by creating a socket handle for IPv6, setting the socket options to be disabled, and then closing the socket.
/// 
pub fn disable_ipv6_socket() -> Result<()> {
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
    if unsafe { libc::setsockopt(ipv6_socket, IPPROTO_IPV6,  IPV6_V6ONLY, &mut ipv6_disabled as *mut _ as *mut c_void,  size_of::<c_int>() as u32) } < 0 {
        eprintln!("Unable to set socket options");
    }

    // Closing ipv6 socket
    unsafe { 
        libc::close(ipv6_socket) 
    };

    Ok(())
}

pub fn disable_ipv6_conf() -> Result<()> {
    // Opening /proc/sys/net/ipv6/conf/all/disable_ipv6
    OpenOptions::new()
        .write(true) // Open file in write mode
        .open("/proc/sys/net/ipv6/conf/all/disable_ipv6")? // If unable to open file, print error message
        .write_all(b"1")?; // Write 1 to /proc/sys/net/ipv6/conf/all/disable_ipv6

    Ok(())
}