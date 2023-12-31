use std::fs::{read_to_string, File};
use std::io::{Error, ErrorKind};
use std::os::unix::process::CommandExt;
use std::path::PathBuf;
use std::process::Command;

const PROC_PATH: &str = "/proc/";

fn restart_service(service_name: &str) -> Result<(), Error> {
    let pid = find_pid_by_name(service_name).map_err(|e| e.to_string())?;
    println!("{} PID is {}", service_name, pid);
 
    let mut command = Command::new("kill");
    command.arg("-HUP").arg(pid.to_string());
 
    match command.exec() {
        Ok(_) => Ok(()),
        Err(err) => Err(err.to_string()),
     }
 }

fn find_pid_by_name(pname: &str) -> Result<i32, Error> {
    let proc_path = PathBuf::from(PROC_PATH);
 
    for entry in proc_path.read_dir()? {
        let entry = entry?;
        let pid_str = entry.file_name().to_string_lossy();
 
        if let Ok(pid) = pid_str.parse::<i32>() {
            let cmdline_path = proc_path.join(pid_str.as_ref()).join("cmdline");
            let cmdline = read_to_string(cmdline_path)?;
 
            if cmdline.contains(pname) {
                return Ok(pid);
            }
        }
    }
 
    Err(Error::new(ErrorKind::NotFound, "Process not found"))
 }