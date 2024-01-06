use std::{process::Command, fs::File};
use std::io::Write;
use color_eyre::eyre::Result;

pub fn disable_auto_mounting() -> Result<()> {
    // Check if the fstab entry already exists
    let output = Command::new("grep")
        .arg("-q")
        .arg("^\\s*Auto\\s*\\(\\s*\\|\\)\\s*\\(no\\|false\\)")
        .arg("/etc/fstab")
        .output()
        .expect("Failed to execute grep command");

    if !output.status.success() {
        // Add the entry to fstab
        let mut fstab = File::create("/etc/fstab").expect("Failed to open fstab file");
        writeln!(fstab, "proc /proc proc defaults,nosuid,nodev,noexec,auto,hidepid=2 0 0")
            .expect("Failed to write to fstab file");
    }

    // Remount the root filesystem as read-only
    let output = Command::new("mount")
        .arg("-o")
        .arg("remount,ro")
        .arg("/")
        .output()
        .expect("Failed to remount filesystem");

    if !output.status.success() {
        eprintln!("Error disabling automounting");
    }

    Ok(())
}