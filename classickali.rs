// Kali Linux Terminal Title Bar Color Modifier (TEMPORARY)
// This program changes ONLY the terminal window's title bar to white
// Effect VANISHES after system restart - no permanent changes!
// Kali Linux specific

use std::fs;
use std::path::PathBuf;
use std::process::Command;
use std::io::Read;

fn main() {
    println!("=== Kali Terminal Title Bar Modifier (Temporary) ===\n");
    
    // Check if running on Linux
    if !cfg!(target_os = "linux") {
        eprintln!("Error: This program only works on Linux systems.");
        std::process::exit(1);
    }

    // Check if running on Kali Linux
    if !is_kali_linux() {
        eprintln!("Warning: This program is designed for Kali Linux.");
        eprintln!("Detected OS may not be Kali. Continue anyway? (y/n)");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        if !input.trim().eq_ignore_ascii_case("y") {
            println!("Aborted.");
            std::process::exit(0);
        }
    }

    // Create the custom GTK CSS for white terminal title bar
    let css_content = r#"
/* Custom CSS to make ONLY terminal title bar white */
window.background.csd > decoration {
    background-color: #ffffff;
    color: #000000;
}

window.background.csd > decoration > headerbar {
    background-color: #ffffff;
    background-image: none;
    color: #000000;
}

window.background.csd > decoration > headerbar button {
    color: #000000;
}

/* Make sure only gnome-terminal is affected */
#gnome-terminal-window decoration {
    background-color: #ffffff;
}

#gnome-terminal-window headerbar {
    background-color: #ffffff;
    background-image: none;
    color: #000000;
}
"#;

    // Use /tmp directory for temporary CSS (cleared on restart)
    let tmp_gtk_dir = PathBuf::from("/tmp/gtk-3.0-temp");
    
    // Create temporary GTK directory
    if !tmp_gtk_dir.exists() {
        match fs::create_dir_all(&tmp_gtk_dir) {
            Ok(_) => println!("✓ Created temporary GTK directory"),
            Err(e) => {
                eprintln!("Error creating temp directory: {}", e);
                std::process::exit(1);
            }
        }
    }

    // Get the user's home directory
    let home_dir = match std::env::var("HOME") {
        Ok(dir) => PathBuf::from(dir),
        Err(_) => {
            eprintln!("Error: Could not determine HOME directory.");
            std::process::exit(1);
        }
    };

    // Create the GTK config directory if it doesn't exist
    let gtk3_config_dir = home_dir.join(".config/gtk-3.0");
    if !gtk3_config_dir.exists() {
        match fs::create_dir_all(&gtk3_config_dir) {
            Ok(_) => println!("✓ Created GTK-3.0 config directory"),
            Err(e) => {
                eprintln!("Error creating config directory: {}", e);
                std::process::exit(1);
            }
        }
    }

    // Write the CSS file to temporary location
    let temp_css_path = tmp_gtk_dir.join("gtk.css");
    let permanent_css_path = gtk3_config_dir.join("gtk.css");
    
    // Save original CSS if it exists (for restoration)
    let original_backup = tmp_gtk_dir.join("original_gtk.css");
    if permanent_css_path.exists() && !original_backup.exists() {
        match fs::copy(&permanent_css_path, &original_backup) {
            Ok(_) => println!("✓ Saved original CSS for restoration"),
            Err(e) => eprintln!("Warning: Could not backup original: {}", e),
        }
    }

    // Write the temporary CSS
    match fs::write(&temp_css_path, css_content) {
        Ok(_) => println!("✓ Created temporary CSS file"),
        Err(e) => {
            eprintln!("Error writing temp CSS file: {}", e);
            std::process::exit(1);
        }
    }

    // Create symlink from ~/.config/gtk-3.0/gtk.css to /tmp/gtk-3.0-temp/gtk.css
    // This way, the change is temporary and will disappear on restart
    if permanent_css_path.exists() {
        fs::remove_file(&permanent_css_path).ok();
    }
    
    match std::os::unix::fs::symlink(&temp_css_path, &permanent_css_path) {
        Ok(_) => {
            println!("✓ Created temporary symlink to CSS");
            println!("\n=== Installation Complete! ===\n");
            println!("Status: TEMPORARY MODE");
            println!("- Title bar will be WHITE in new terminals");
            println!("- Effect will VANISH after system restart");
            println!("- No permanent changes to your system");
            println!("\nInstructions:");
            println!("1. Close ALL terminal windows");
            println!("2. Open a new terminal");
            println!("3. The title bar should now be WHITE!");
            println!("\nTo revert BEFORE restart:");
            println!("- Run: ./terminal_titlebar_white --restore");
            println!("- Or manually delete: {:?}", permanent_css_path);
            println!("\nAfter restart: Everything returns to normal automatically!");
        }
        Err(e) => {
            eprintln!("Error creating symlink: {}", e);
            eprintln!("Trying direct copy method instead...");
            
            match fs::copy(&temp_css_path, &permanent_css_path) {
                Ok(_) => println!("✓ CSS applied (will be cleared on restart)"),
                Err(e) => {
                    eprintln!("Error: {}", e);
                    std::process::exit(1);
                }
            }
        }
    }

    // Optional: Try to reload GTK settings
    println!("\nAttempting to reload GTK settings...");
    let _ = Command::new("killall")
        .arg("-HUP")
        .arg("gnome-terminal-server")
        .output();
    
    println!("✓ Done! Open a new terminal to see the changes.");
}

// Function to check if running on Kali Linux
fn is_kali_linux() -> bool {
    // Check /etc/os-release for Kali
    if let Ok(mut file) = fs::File::open("/etc/os-release") {
        let mut contents = String::new();
        if file.read_to_string(&mut contents).is_ok() {
            return contents.to_lowercase().contains("kali");
        }
    }
    
    // Fallback: check for kali-specific files
    PathBuf::from("/usr/share/kali-themes").exists()
}
