use std::fs;
use std::io::ErrorKind;
use std::os::unix::fs::MetadataExt;
use std::os::unix::io::AsRawFd;
use nix::mount::{mount, MsFlags};

const TMP_DIR: &str = "/tmp";

pub fn ensure_nosuid_on_tmp() -> Result<(), std::io::Error> {
    // Remount /tmp with nosuid option
    mount(
        Some(TMP_DIR),
        TMP_DIR,
        None, // Use existing filesystem type
        MsFlags::MS_NOATIME | MsFlags::MS_NOSUID | MsFlags::MS_NODEV,
        None,
    )?;

    // Check if nosuid option is set
    let metadata = fs::metadata(TMP_DIR)?;
    if !metadata.st_flags().contains(nix::sys::stat::SFlag::ST_NOSUID) {
        return Err(std::io::Error::new(
            ErrorKind::Other,
            "nosuid option is not set for /tmp",
        ));
    }

    Ok(())
}
