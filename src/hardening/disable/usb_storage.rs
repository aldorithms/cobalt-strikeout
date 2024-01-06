use std::io::Write;
use color_eyre::Result;

pub fn disable_loading_usb_storage() -> Result<()> {
    std::fs::OpenOptions::new()
        .write(true) // Open file in write mode
        .open("/etc/modprobe.d/blacklist-usb-storage.conf")? // Open file by path
        .write_all(b"blacklist usb-storage\n")?; // Write 1 to file

    Ok(())
}