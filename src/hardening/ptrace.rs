use color_eyre::Result;
use prctl::set_dumpable;

pub fn disable_ptrace() -> Result<()> {
    // Disable ptrace
    set_dumpable(false)
        .expect("Unable to disable ptrace");
    Ok(())
}