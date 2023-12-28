use cobalt_strikeout::system::system_update;
use cobalt_strikeout::hardening::{disable_core_dumps, disable_ipv6_socket};
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
    
    Ok(())
}