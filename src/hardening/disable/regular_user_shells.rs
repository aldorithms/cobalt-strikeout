use std::ffi::CStr;
use std::process::Command;
use libc::{getpwuid, uid_t};
use color_eyre::Result;

pub fn disable_regular_user_shells() -> Result<()> {
    const RESTRICTED_SHELL: &str = "/bin/rbash";
    const START_UID: uid_t = 1000;
    const END_UID: uid_t = 65535;

    for uid in START_UID..=END_UID {
        let pw = unsafe{ getpwuid(uid) };

        if pw.is_null() {
            continue; // User not found, skip to next user
        }

        let pw_name = unsafe { CStr::from_ptr((*pw).pw_name) }.to_str().unwrap();
        if pw_name != "root" {
            let output = Command::new("chsh")
                .arg("-s")
                .arg(RESTRICTED_SHELL)
                .arg(pw_name)
                .output()
                .expect("Failed to execute chsh command");

                if !output.status.success() {
                    eprintln!("Failed to change shell for user {}: {}", pw_name, String::from_utf8_lossy(&output.stderr));
                }
        }
    }
    Ok(())
}