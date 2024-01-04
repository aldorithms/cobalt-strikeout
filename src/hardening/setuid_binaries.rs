use prctl::set_no_new_privileges;
use color_eyre::Result;

pub fn disable_setuid_binaries() -> Result<()> {
    // Disable the ability to use setuid binaries
    set_no_new_privileges(true)
        .expect("Unable to disable setuid binaries");
    Ok(())
}