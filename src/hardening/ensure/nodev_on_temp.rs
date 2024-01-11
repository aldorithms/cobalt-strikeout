use std::fs::File;
use std::io::{BufRead, BufReader, ErrorKind, Read};

pub fn ensure_nodev_on_temp() -> Result<(), std::io::Error> {
    // Open mountinfo file
    let mountinfo = File::open("/proc/self/mountinfo")?;
    let reader = BufReader::new(mountinfo);

    // Check each line for /tmp mount with nodev option
    for line in reader.lines() {
        let line = line?;
        if let Some(mount_point) = line.find(" /tmp ") {
            if let Some(options) = line.rsplit(" - ").next() {
                if options.contains("nodev") {
                    println!("nodev option is set for /tmp");
                    return Ok(());
                }
            }
        }
    }

    Err(std::io::Error::new(
        ErrorKind::NotFound,
        "nodev option is not set for /tmp",
    ))
}
