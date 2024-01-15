use std::fs::{File, OpenOptions};
use std::io::{BufReader, BufWriter, Error, ErrorKind, Write, SeekFrom, Seek, BufRead};
use std::os::unix::fs::{PermissionsExt, chown,};
use std::path::Path;



// Function to encapsulate permission-setting logic with error handling
fn set_permissions(path: &Path, owner: u32, group: u32, mode: u32) -> Result<(), Error> {
    chown(path, owner, group)?;
    chmod(path, mode)?;
    Ok(())
}

fn audit_file_permissions() {
    // Set permissions for /etc/passwd
    set_permissions(Path::new("/etc/passwd"), 0, 0, 0o644)?;
    set_permissions(Path::new("/etc/passwd-"), 0, 0, 0o666)?;

    // Set permissions for /etc/group
    set_permissions(Path::new("/etc/group"), 0, 0, 0o644)?;
    set_permissions(Path::new("/etc/group-"), 0, 0, 0o644)?;

    // Set permissions for /etc/shadow and /etc/shadow-
    set_permissions(Path::new("/etc/shadow"), 0, 2, 0o640)?;
    set_permissions(Path::new("/etc/shadow-"), 0, 2, 0o640)?;

    // Get user and group information
    let pwd = users::get_user_by_name("root")?;
    let grp = groups::get_group_by_name("shadow")?;

    // Set permissions for /etc/gshadow and /etc/gshadow-
    set_permissions(Path::new("/etc/gshadow"), pwd.uid(), grp.gid(), 0o640)?;
    set_permissions(Path::new("/etc/gshadow-"), pwd.uid(), grp.gid(), 0o640)?;
}
