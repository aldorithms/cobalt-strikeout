use std::fs::read_to_string;
use std::io::{Error, ErrorKind};
use std::process::{Command, Stdio};
use std::path::PathBuf;
use color_eyre::Result;

const PROC_PATH: &str = "/proc/";

pub fn restart_service(service_name: &str) -> Result<()> {
    let pid = find_pid_by_name(service_name)?;
    println!("{} PID is {}", service_name, pid);

    let mut command = Command::new("kill")
        .arg("-HUP")
        .arg(pid.to_string())
        .stdout(Stdio::piped())
        .spawn()?
        .wait_with_output()?;

    Ok(())
}

fn find_pid_by_name(pname: &str) -> Result<i32, Error> {
    let proc_path = PathBuf::from(PROC_PATH);

    for entry in proc_path.read_dir()? {
        let entry = entry?;
        let pid_str = entry.file_name().to_string_lossy().into_owned();

        if let Ok(pid) = pid_str.parse::<i32>() {
            let cmdline_path = proc_path.join(&*pid_str).join("cmdline");
            let cmdline = read_to_string(cmdline_path)?;

            if cmdline.contains(pname) {
                return Ok(pid);
            }
        }
    }

    Err(Error::new(ErrorKind::NotFound, "Process not found"))
}
