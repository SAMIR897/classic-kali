use std::process::Command;
use std::env;
use std::fs;
use std::path::Path;

fn main() {
    println!("[*] Initializing session-only retro Kali modifications...");

    if let Ok(home) = env::var("HOME") {
        let terminal_config = format!("{}/.config/xfce4/terminal/terminalrc", home);
        let terminal_backup = format!("{}/.config/xfce4/terminal/terminalrc.bak", home);
        let autostart_dir = format!("{}/.config/autostart", home);
        let autostart_file = format!("{}/retro-revert.desktop", autostart_dir);

        // 1. BACKUP ORIGINAL CONFIG (Only if a backup doesn't already exist)
        if !Path::new(&terminal_backup).exists() {
            if Path::new(&terminal_config).exists() {
                let _ = fs::copy(&terminal_config, &terminal_backup);
            }
        }

        // 2. APPLY RETRO WINDOW BORDERS (Chunkier white title bar)
        let _ = Command::new("xfconf-query")
            .args(["-c", "xfwm4", "-p", "/general/theme", "-s", "Crux"])
            .status();

        // 3. APPLY TERMINAL COLORS
        if Path::new(&terminal_config).exists() {
            let _ = Command::new("sed").args(["-i", "s/^ColorUseTheme=.*/ColorUseTheme=FALSE/", &terminal_config]).status();
            let _ = Command::new("sed").args(["-i", "s/^ColorBackground=.*/ColorBackground=#000000/", &terminal_config]).status();
            let _ = Command::new("sed").args(["-i", "s/^ColorForeground=.*/ColorForeground=#ffffff/", &terminal_config]).status();
        }

        // 4. CREATE THE "SELF-DESTRUCTING" AUTO-REVERT SCRIPT FOR NEXT BOOT
        // This ensures that when you restart, everything goes back to normal, and this file deletes itself.
        let _ = fs::create_dir_all(&autostart_dir); // Ensure autostart dir exists
        
        let autostart_payload = format!(
            "[Desktop Entry]\n\
            Type=Application\n\
            Name=Retro Theme Reverter\n\
            Exec=sh -c \"xfconf-query -c xfwm4 -p /general/theme -s Kali-Dark && mv {} {} && rm {}\"\n\
            Hidden=false\n\
            NoDisplay=false\n\
            X-GNOME-Autostart-enabled=true\n",
            terminal_backup, terminal_config, autostart_file
        );

        match fs::write(&autostart_file, autostart_payload) {
            Ok(_) => {
                println!("[+] Session-only retro mode activated.");
                println!("[+] Any new terminal you open will use the classic look.");
                println!("[+] A self-destructing reset trigger has been set for your next reboot.");
            }
            Err(e) => eprintln!("[-] Failed to create auto-revert trigger: {}", e),
        }
    } else {
        eprintln!("[-] Could not locate HOME directory.");
    }
}
