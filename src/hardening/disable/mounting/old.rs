pub fn disable_freevxfs_mounting() {
    // Check if the freevxfs module is loaded
    if is_module_loaded("freevxfs") {
        println!("freevxfs module is loaded, removing...");
        if let Err(e) = remove_module("freevxfs") {
            eprintln!("Error removing module: {}", e);
        }
    }

    // Prevent future loading of the freevxfs module
    if let Err(e) = create_modprobe_conf("/etc/modprobe.d/freevxfs.conf", "install freevxfs /bin/true") {
        eprintln!("Error creating modprobe configuration: {}", e);
    }
}

/// Description - The jffs2 (journaling flash filesystem 2) filesystem type is a log-structured filesystem.
/// It is used in flash memory devices.
pub fn disable_jffs_mounting() {
    // Check if the jffs2 module is loaded
    if is_module_loaded("jffs2") {
        println!("jffs2 module is loaded, removing...");
        if let Err(e) = remove_module("jffs2") {
            eprintln!("Error removing module: {}", e);
        }
    }

    // Prevent future loading of the jffs2 module
    if let Err(e) = create_modprobe_conf("/etc/modprobe.d/jffs2.conf", "install jffs2 /bin/true") {
        eprintln!("Error creating modprobe configuration: {}", e);
    }
}

/// Description - The hfs filesystem type is a hierarchical filesystem that allows you to mount Mac OS filesystems.
pub fn disable_hfs_mounting() {
    // Check if the hfs module is loaded
    if is_module_loaded("hfs") {
        println!("hfs module is loaded, removing...");
        if let Err(e) = remove_module("hfs") {
            eprintln!("Error removing module: {}", e);
        }
    }

    // Prevent future loading of the hfs module
    if let Err(e) = create_modprobe_conf("/etc/modprobe.d/hfs.conf", "install hfs /bin/true") {
        eprintln!("Error creating modprobe configuration: {}", e);
    }
}

/// Description - The hfsplus filesystem type is a hierarchical filesystem that allows you to mount Mac OS filesystems.
pub fn disable_hfsplus_mounting() {
    // Check if the hfsplus module is loaded
    if is_module_loaded("hfsplus") {
        println!("hfsplus module is loaded, removing...");
        if let Err(e) = remove_module("hfsplus") {
            eprintln!("Error removing module: {}", e);
        }
    }

    // Prevent future loading of the hfsplus module
    if let Err(e) = create_modprobe_conf("/etc/modprobe.d/hfsplus.conf", "install hfsplus /bin/true") {
        eprintln!("Error creating modprobe configuration: {}", e);
    }
}

/// Description - The udf filesystem type is the universal disk format used to implement ISO/IEC 13346 andECMA-167 specifications. 
/// This is an open vendor filesystem type for data storage on a broad range of media. 
/// This filesystem type is necessary to support writing DVDs and newer optical disc formats.
pub fn disable_udf_mounting() {
    // Check if the udf module is loaded
    if is_module_loaded("udf") {
        println!("udf module is loaded, removing...");
        if let Err(e) = remove_module("udf") {
            eprintln!("Error removing module: {}", e);
        }
    }

    // Prevent future loading of the udf module
    if let Err(e) = create_modprobe_conf("/etc/modprobe.d/udf.conf", "install udf /bin/true") {
        eprintln!("Error creating modprobe configuration: {}", e);
    }
}

fn is_module_loaded(module_name: &str) -> bool {
    let output = Command::new("lsmod")
        .stdout(Stdio::piped())
        .output()
        .expect("Failed to execute lsmod");

    let stdout = String::from_utf8_lossy(&output.stdout);

    stdout.lines().any(|line| line.contains(module_name))
}

fn remove_module(module_name: &str) -> Result<()> {
    Command::new("rmmod")
        .arg(module_name)
        .spawn()?
        .wait()?;

    Ok(())
}

fn create_modprobe_conf(path: &str, content: &str) -> Result<()> {
    let mut file = File::create(path)?;

    file.write_all(content.as_bytes())?;

    Ok(())
}