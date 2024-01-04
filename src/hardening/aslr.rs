use rlimit::{getrlimit, setrlimit, Resource};
use color_eyre::Result;

pub fn enable_aslr() -> Result<()> {
    if getrlimit(Resource::STACK)? == (0, 0) {
        setrlimit(Resource::STACK, rlimit::INFINITY, rlimit::INFINITY)?;
    }
    Ok(())
}