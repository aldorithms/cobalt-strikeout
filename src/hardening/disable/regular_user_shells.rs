use std::os::unix::process::CommandExt;
use std::process::Command;

fn disable_regular_user_shells() {
   const RESTRICTED_SHELL: &str = "/bin/rbash";
   const START_UID: u32 = 1000;
   const END_UID: u32 = 65535;

   for uid in START_UID..=END_UID {
       match nix::unistd::getpwuid(uid) {
           Ok(Some(pw)) => {
               if pw.pw_name != "root" {
                   let output = Command::new("chsh")
                       .arg("-s")
                       .arg(RESTRICTED_SHELL)
                       .arg(pw.pw_name)
                       .output()
                       .expect("Failed to execute chsh command");

                   if !output.status.success() {
                       eprintln!("Failed to change shell for user {}: {}", pw.pw_name, String::from_utf8_lossy(&output.stderr));
                   }
               }
           }
           Ok(None) => {} // User not found, skip
           Err(e) => eprintln!("Error getting user information for UID {}: {}", uid, e),
       }
   }
}
