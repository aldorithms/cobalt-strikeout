use std::fs::{File, OpenOptions};
use std::io::{BufReader, BufWriter, Error, ErrorKind, Write, SeekFrom, Seek, BufRead};
use std::os::unix::fs::{PermissionsExt, chown,};
use std::path::Path;

const MAX_LINE_LEN: usize = 256;

pub fn audit_password_policy() -> Result<(), Error> {
    let path = Path::new("/etc/pam.d/common-password");
    let mut file = OpenOptions::new().read(true).write(true).create(true).open(path)?;
    let mut reader = BufReader::new(&file);

    let mut line = String::new();
    while reader.read_line(&mut line)? > 0 {
        if line.contains("pam_pwquality.so") {
            return Err(Error::new(ErrorKind::AlreadyExists, "Password policy configuration already exists in file"));
        }
        line.clear(); // Clear the line for the next iteration
    }

    // Necessary to reposition the cursor for writing after reading
    file.seek(SeekFrom::End(0))?;

    writeln!(&file, "password   required   pam_pwquality.so try_first_pass retry=3")?;

    let mut pwquality_conf_file = File::create("/etc/security/pwquality.conf")?;
    writeln!(pwquality_conf_file, "minlen = 12")?;
    writeln!(pwquality_conf_file, "dcredit = -1")?;
    writeln!(pwquality_conf_file, "ucredit = -1")?;
    writeln!(pwquality_conf_file, "ocredit = -1")?;

    Ok(())
}