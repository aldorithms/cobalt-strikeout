use std::process::{Command, Stdio};
use std::io::Write;
use std::fs::File;

pub fn disable_freevxfs_mounting() {
    // Check if the freevxfs module is loaded
    if is_module_loaded("freevxfs") {
        println!("freevxfs module is loaded, removing...");
        if let Err(e) = remove_module("freevxfs") {
            eprintln!("Error removing module: {}", e);
        }
    }

    // Prevent future loading of the freevxfs module
    if let Err(e) = create_modprobe_conf("/etc/modprobe.d/freevxfs.conf", "install freevxfs /bin/true") {
        eprintln!("Error creating modprobe configuration: {}", e);
    }
}

fn is_module_loaded(module_name: &str) -> bool {
    let output = Command::new("lsmod")
        .stdout(Stdio::piped())
        .output()
        .expect("Failed to execute lsmod");

    let stdout = String::from_utf8_lossy(&output.stdout);
    stdout.lines().any(|line| line.contains(module_name))
}

fn remove_module(module_name: &str) -> std::io::Result<()> {
    Command::new("rmmod")
        .arg(module_name)
        .spawn()?
        .wait()?;
    Ok(())
}

fn create_modprobe_conf(path: &str, content: &str) -> std::io::Result<()> {
    let mut file = File::create(path)?;
    file.write_all(content.as_bytes())?;
    Ok(())
}