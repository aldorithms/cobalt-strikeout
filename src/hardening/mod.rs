pub mod disable;
pub mod enable;
pub mod ensure;
pub mod ssh;
pub mod restart_service;

/*
void disable_core_dumps();                      done
void disable_ipv6();                        done
void disable_setuid_binaries();            done 
void disable_ptrace();                      done
void disable_loading_kernel_modules();   done
void disable_loading_USB_Storage();         done
void disable_freevxfs_mounting();       done
void disable_jffs_mounting();           done
void disable_hfs_mounting();            done
void disable_hfsplus_mounting();        done
void disable_udf_mounting();            done
void disable_auto_mounting();               done
void disable_packet_redirect_sending();     done
void disable_ip_forwarding();               done
void disable_source_routing();          done      
void disable_icmp_redirects();              done
void disable_regular_user_shells();

void harden_sshd();                     done
void secure_grub();


void enable_aslr();                     done
void ensure_tmp_is_configured();
void ensure_nodev_on_temp();
void ensure_nosuid_on_tmp();
void ensure_shm();
void ensure_nosuid_on_shm();
void ensure_sticky_bit();
void enable_tcp_syn_cookies();
void ensure_sudo_uses_pty();
void ensure_sudo_log_file_exists();

void secure_samba();
void secure_mysql();
void secure_database_services();

void reverse_linpeas();

void remove_netrc_files();
*/