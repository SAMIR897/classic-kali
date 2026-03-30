# Project: Classic Kali (Rust Edition)

> *"Before the dark themes, before the non-root defaults, there was a glaring white window frame and a pure black terminal."*

For those who remember their high school days staying up till 3 AM running `aircrack-ng` and `nmap`—when Kali Linux (and BackTrack before it) booted straight to `root`, and every terminal window was wrapped in a chunky, unforgiving white GTK border. 

This project brings that exact 2016 aesthetic back to modern Kali Linux. We are resurrecting the classic hacker vibe, but we are doing it with the speed, safety, and precision of modern **Rust**.

---

## ⚙️ The Concept

Modern Kali Linux ships with a sleek, unified, borderless "Kali-Dark" XFCE theme. While beautiful and easy on the eyes, it lacks the raw, utilitarian feel of the older builds. 

**Classic Kali** is a lightweight, compiled Rust binary that acts as a temporary, session-based theme injector. It surgically alters your XFCE Window Manager to bring back the thick white title bars and forces your default terminal into a strict `#ffffff` on `#000000` (White on Black) colorway. 

The best part? **It is entirely volatile.** No background daemons, no bloated processes, and no permanent damage to your meticulously crafted dark mode. The moment you reboot, the script self-destructs its payload, and your system reverts to its stealthy modern state.

---

## 🚀 Features

* **Targeted Nostalgia:** Modifies *only* the window borders (`xfwm4`) and terminal colors (`terminalrc`). Your file manager, menus, and core system applications remain safely in dark mode to protect your eyes.
* **Zero-Daemon Architecture:** This is not a background process. The Rust binary executes in milliseconds, modifies the necessary configuration files, sets a trap for the next boot, and cleanly exits. Zero RAM overhead.
* **The "Ghost" Autostart:** Automatically generates a self-destructing `.desktop` trigger in your `~/.config/autostart/` directory. On your next reboot or login, the system restores your original terminal backups, resets the dark theme, and deletes the trigger file without a trace.
* **Blazing Fast:** Written in standard Rust. Compiled once, runs instantly.

---

## 🛠️ Installation & Compilation

**Prerequisites:** You must be running Kali Linux with its default **XFCE** desktop environment. You also need the Rust compiler (`rustc`).

1. **Install Rust** (if not already installed):
   ```bash
   sudo apt update
   sudo apt install rustc
