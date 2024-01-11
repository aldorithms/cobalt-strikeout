use std::fs;
use std::io::prelude::*;
use std::os::unix::fs::PermissionsExt;
use std::path::Path;

const TMP_DIR: &str = "/tmp";

pub fn ensure_tmp_is_configured() -> Result<(), std::io::Error> {
    // Check if /tmp exists and is a directory
    if !Path::new(TMP_DIR).is_dir() {
        return Err(std::io::Error::new(std::io::ErrorKind::NotFound, "/tmp does not exist or is not a directory"));
    }

    // Set permissions to 1777 with sticky bit
    fs::set_permissions(TMP_DIR, fs::Permissions::from_mode(0o1777))?;

    // Create tmpfiles.d configuration file if it doesn't exist
    let conf_path = Path::new("/etc/tmpfiles.d/tmp.conf");
    if !conf_path.exists() {
        let mut conf = fs::File::create(conf_path)?;
        conf.write_all(b"D /tmp 1777 root root 10d\n")?;
    }

    Ok(()) // Return Ok to indicate success
}
