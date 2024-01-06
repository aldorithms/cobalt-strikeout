use std::io::Write;
use color_eyre::Result;

pub fn disable_kernel_modules() -> Result<()> {
    // Disable loading of kernel modules
    std::fs::OpenOptions::new()
        .write(true) // Open file in write mode
        .open("/proc/sys/kernel/modules_disabled")? // Open file by path
        .write_all(b"1")?; // Write 1 to file

    Ok(())
}