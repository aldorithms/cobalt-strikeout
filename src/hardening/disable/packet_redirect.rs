use std::fs::File;
use std::io::{ Write, Read };
use color_eyre::Result;


/// # Description
/// ICMP Redirects are used to send routing information to other hosts. 
/// As a host itself does not act as a router (in a host only configuration), there is no need to send redirects

/// # Rationale
/// An attacker could use a compromised host to send invalid ICMP redirects to other router devices
/// in an attempt to corrupt routing and have users access a system set up by the attacker as opposed to a valid system.
pub fn disable_packet_redirect_sending() -> Result<()> {
    // Modify /etc/sysctl.conf
    {
        let mut file = File::open("/etc/sysctl.conf")?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        let mut send_redirects_enabled = false;
        for line in contents.lines() {
            if line.contains("net.ipv4.conf.all.send_redirects") {
                if line.contains("= 1") {
                    send_redirects_enabled = true;
                }
                break;
            }
        }

        if send_redirects_enabled {
            contents.push_str("\nnet.ipv4.conf.all.send_redirects=0");
            file.write_all(contents.as_bytes())?;
        }
    }

    // Write to /proc/sys/net/ipv4/conf/all/send_redirects
    {
        let mut file = File::create("/proc/sys/net/ipv4/conf/all/send_redirects")?;
        file.write_all(b"0\n")?;
    }

    // Modify /etc/ssh/sshd_config
    {
        let mut file = File::open("/etc/ssh/sshd_config")?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        let mut modified = false;
        for mut line in contents.lines() {
            if line.starts_with("#UsePAM") || line.starts_with("UsePAM") {
                line = "UsePAM yes\n";
                modified = true;
                break;
            }
        }

        if modified {
            file.write_all(contents.as_bytes())?;
        }
    }

    Ok(())
}
