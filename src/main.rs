use cobalt_strikeout::hardening::disable::lkm::disable_loading_kernel_modules;
use cobalt_strikeout::hardening::disable::mounting::auto::disable_auto_mounting;
use cobalt_strikeout::hardening::disable::packet_redirect::disable_packet_redirect_sending;
use cobalt_strikeout::hardening::disable::ptrace::disable_ptrace;
use cobalt_strikeout::hardening::disable::setuid_binaries::disable_setuid_binaries;
use cobalt_strikeout::hardening::disable::usb_storage::disable_loading_usb_storage;
use cobalt_strikeout::system::system_update;
use cobalt_strikeout::hardening::disable::{core_dumps::disable_core_dumps, ipv6::{disable_ipv6_socket, disable_ipv6_conf}};
use color_eyre::Result;

fn main() -> Result<()> {
    match system_update() {
        Ok(_) => println!("System updated successfully"),
        Err(e) => eprintln!("Error updating system: {}", e),
    };
    match disable_core_dumps() {
        Ok(_) => println!("Core dumps disabled successfully"),
        Err(e) => eprintln!("Error disabling core dumps: {}", e),
    };
    match disable_ipv6_socket() {
        Ok(_) => println!("IPv6 socket disabled successfully"),
        Err(e) => eprintln!("Error disabling IPv6 socket: {}", e),
    };
    match disable_ipv6_conf() {
        Ok(_) => println!("IPv6 routing disabled successfully"),
        Err(e) => eprintln!("Error disabling IPv6 routing: {}", e),
    };
    match disable_setuid_binaries() {
        Ok(_) => println!("Setuid binaries disabled successfully"),
        Err(e) => eprintln!("Error disabling setuid binaries: {}", e),
    };
    match disable_ptrace() {
        Ok(_) => println!("Ptrace disabled successfully"),
        Err(e) => eprintln!("Error disabling ptrace: {}", e),
    };
    match disable_loading_kernel_modules() {
        Ok(_) => println!("Kernel modules disabled successfully"),
        Err(e) => eprintln!("Error disabling kernel modules: {}", e),
    };
    match disable_loading_usb_storage() {
        Ok(_) => println!("USB storage disabled successfully"),
        Err(e) => eprintln!("Error disabling USB storage: {}", e),
    };
    match disable_auto_mounting() {
        Ok(_) => println!("Auto mounting disabled successfully"),
        Err(e) => eprintln!("Error disabling auto mounting: {}", e),
    };
    match disable_packet_redirect_sending() {
        Ok(_) => println!("Packet redirect sending disabled successfully"),
        Err(e) => eprintln!("Error disabling packet redirect sending: {}", e),
    };
    Ok(())
}