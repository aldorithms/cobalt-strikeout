use rlimit::{setrlimit, Resource};
use color_eyre::Result;

pub fn disable_core_dumps() -> Result<()> {
    setrlimit(Resource::CORE, 0, 0)?;

    Ok(())
}