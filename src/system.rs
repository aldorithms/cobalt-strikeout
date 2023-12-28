use std::process::Command;

enum PackageManager {
    None,
    AptGet,
    Dnf,
    Yum,
    Apk,
    Pacman,
    SlaptGet,
    SlackPkg,
}

fn is_package_manager_installed(pkg_mngr: &str) -> bool {
    which::which(pkg_mngr).is_ok()
}

fn get_package_manager() -> PackageManager {
    let mut package_manager = PackageManager::None;
    let pkg_mngrs = [ "apt-get", "dnf", "yum", "apk", "pacman", "slapt-get", "slackpkg",];

    for &pkg_mngr in pkg_mngrs.iter() {
        if is_package_manager_installed(pkg_mngr) {
            package_manager = match pkg_mngr {
                "apt-get" => PackageManager::AptGet, // For Debian
                "dnf" => PackageManager::Dnf, // For Fedora
                "yum" => PackageManager::Yum, // For CentOS
                "apk" => PackageManager::Apk, // For Alpine Linux
                "pacman" => PackageManager::Pacman, // For Arch Linux
                "slapt-get" => PackageManager::SlaptGet, // For Slackware
                "slackpkg" => PackageManager::SlackPkg, // For Slackware
                _ => PackageManager::None,
            };
        }
    }

    package_manager
}

pub fn system_update() -> Result<(), std::io::Error> {
    println!("Updating system...");
    match get_package_manager() {
        PackageManager::AptGet => {
            Command::new("apt-get")
                .args("update -y && apt-get upgrade -y".split_ascii_whitespace())
                .status()
                .expect("failed to update system");
        }
        PackageManager::Dnf => {
            Command::new("dnf")
                .args("update -y && apk upgrade -y".split_ascii_whitespace())
                .status()
                .expect("failed to update system");
        }
        PackageManager::Yum => {
            Command::new("yum")
                .args("update -y".split_ascii_whitespace())
                .status()
                .expect("failed to update system");
        }
        PackageManager::Apk => {
            Command::new("apk")
                .args("update && apk upgrade".split_ascii_whitespace())
                .status()
                .expect("failed to update system");
        }
        PackageManager::Pacman => {
            Command::new("pacman")
                .args("-Syu --noconfirm".split_ascii_whitespace())
                .status()
                .expect("failed to update system");
        }
        PackageManager::SlaptGet => {
            Command::new("slapt-get")
                .args("update -y && slapt-get upgrade -y".split_ascii_whitespace())
                .status()
                .expect("failed to update system");
        }
        PackageManager::SlackPkg => {
            Command::new("slackpkg")
                .args("update -y && slackpkg upgrade-all -y".split_ascii_whitespace())
                .status()
                .expect("failed to update system");
        }
        _ => {
            println!("No package manager found.");
        }
    }

    Ok(())
    
}