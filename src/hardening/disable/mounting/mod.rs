pub mod auto;

use std::process::{Command, Stdio};
use std::io::Write;
use std::fs::File;
use color_eyre::eyre::Result;

pub fn disable_fs_mounting(filesystem: &str) {
    if is_module_loaded(filesystem) {
        println!("{} module is loaded, removing...", filesystem);
        if let Err(e) = remove_module(filesystem) {
            eprintln!("Error removing module: {}", e);
        }
    }

    // Prevent future loading of the module
    if let Err(e) 
        = create_modprobe_conf(
            format!("/etc/modprobe.d/{}.conf", filesystem).as_str(), 
            format!("install {} /bin/true", filesystem).as_str()
        ) {
            eprintln!("Error creating modprobe configuration: {}", e);
        }
}

/*-----------------------------------------*/
//    Private Function Section.
/*-----------------------------------------*/

fn is_module_loaded(module_name: &str) -> bool {
    let output = Command::new("lsmod")
        .stdout(Stdio::piped())
        .output()
        .expect("Failed to execute lsmod");

    let stdout = String::from_utf8_lossy(&output.stdout);

    stdout.lines().any(|line| line.contains(module_name))
}

fn remove_module(module_name: &str) -> Result<()> {
    Command::new("rmmod")
        .arg(module_name)
        .spawn()?
        .wait()?;

    Ok(())
}

fn create_modprobe_conf(path: &str, content: &str) -> Result<()> {
    let mut file = File::create(path)?;

    file.write_all(content.as_bytes())?;

    Ok(())
}