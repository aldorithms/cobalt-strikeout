use nix::mount::MsFlags; // Import necessary constants from nix
use nix::mount::mount;
use color_eyre::Result;

const TMP_DIR: &str = "/tmp";

pub fn ensure_nosuid_on_tmp() -> Result<()> {
    // Remount /tmp with nosuid option
    mount::<str, str, str, str>(
        Some(TMP_DIR),
        TMP_DIR,
        None, // Use existing filesystem type
        MsFlags::MS_NOATIME | MsFlags::MS_NOSUID | MsFlags::MS_NODEV,
        None,
     )?;

    // edit: no need to check since function will return error if it fails.
    Ok(())
}
