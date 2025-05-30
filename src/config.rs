use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::PathBuf;

pub const CONFIG_DIR: &str = ".config/better-terminal";
pub const CONFIG_FILE: &str = "better-terminal.conf";

pub fn get_config_path() -> Option<PathBuf> {
    dirs::home_dir().map(|mut path| {
        path.push(CONFIG_DIR);
        path.push(CONFIG_FILE);
        path
    })
}

pub fn load_title_bar_setting() -> bool {
    if let Some(config_path) = get_config_path() {
        if config_path.exists() {
            if let Ok(mut file) = File::open(config_path) {
                let mut contents = String::new();
                if file.read_to_string(&mut contents).is_ok() {
                    for line in contents.lines() {
                        if line.starts_with("titlebar") {
                            let parts: Vec<&str> = line.split('=').map(|s| s.trim()).collect();
                            if parts.len() == 2 && parts[0] == "titlebar" {
                                return parts[1] == "true";
                            }
                        }
                    }
                }
            }
        }
    }
    true
}

pub fn save_title_bar_setting(is_visible: bool) {
    if let Some(config_path) = get_config_path() {
        if let Some(parent_dir) = config_path.parent() {
            if !parent_dir.exists() {
                if let Err(e) = fs::create_dir_all(parent_dir) {
                    eprintln!("Failed to create config directory: {}", e);
                    return;
                }
            }
        }
        
        let mut config_content = String::new();
        if config_path.exists() {
            if let Ok(mut file) = File::open(&config_path) {
                if file.read_to_string(&mut config_content).is_err() {
                    eprintln!("Failed to read existing config file, will overwrite.");
                    config_content.clear();
                }
            }
        }

        let mut new_lines = Vec::new();
        let mut titlebar_found = false;
        for line in config_content.lines() {
            if line.starts_with("titlebar") {
                new_lines.push(format!("titlebar = {}", is_visible));
                titlebar_found = true;
            } else {
                new_lines.push(line.to_string());
            }
        }

        if !titlebar_found {
            new_lines.push(format!("titlebar = {}", is_visible));
        }

        if let Ok(mut file) = File::create(config_path) {
            if let Err(e) = file.write_all(new_lines.join("\n").as_bytes()) {
                eprintln!("Failed to write to config file: {}", e);
            }
        } else {
            eprintln!("Failed to create or open config file for writing.");
        }
    }
}
