use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::PathBuf;

pub const CONFIG_DIR: &str = ".config/better-terminal";
pub const CONFIG_FILE: &str = "better-terminal.conf";

#[derive(Debug, Clone, PartialEq)]
pub enum ColorSchemePreset {
    GruvboxDark,
    CatppuccinMocha,
}

impl ColorSchemePreset {
    pub fn name(&self) -> &'static str {
        match self {
            ColorSchemePreset::GruvboxDark => "GruvboxDark",
            ColorSchemePreset::CatppuccinMocha => "CatppuccinMocha",
        }
    }

    pub fn from_name(name: &str) -> Option<Self> {
        match name {
            "GruvboxDark" => Some(ColorSchemePreset::GruvboxDark),
            "CatppuccinMocha" => Some(ColorSchemePreset::CatppuccinMocha),
            _ => None,
        }
    }

    pub fn all_presets() -> Vec<Self> {
        vec![ColorSchemePreset::GruvboxDark, ColorSchemePreset::CatppuccinMocha]
    }
}

#[derive(Debug, Clone)]
pub struct ColorSettings {
    pub foreground: Option<String>,
    pub background: Option<String>,
    pub palette: Vec<Option<String>>,
    pub active_preset: Option<String>,
}

impl Default for ColorSettings {
    fn default() -> Self {
        ColorSettings {
            foreground: None,
            background: None,
            palette: vec![None; 16],
            active_preset: None,
        }
    }
}

pub fn get_preset_colors(preset: &ColorSchemePreset) -> ColorSettings {
    let mut settings = ColorSettings::default();
    settings.active_preset = Some(preset.name().to_string());
    match preset {
        ColorSchemePreset::GruvboxDark => {
            settings.background = Some("rgba(40, 40, 40, 1.0)".to_string());
            settings.foreground = Some("rgba(235, 219, 178, 1.0)".to_string());
            settings.palette = vec![
                Some("rgba(40, 40, 40, 1.0)".to_string()),
                Some("rgba(204, 36, 29, 1.0)".to_string()),
                Some("rgba(152, 151, 26, 1.0)".to_string()),
                Some("rgba(215, 153, 33, 1.0)".to_string()),
                Some("rgba(69, 133, 136, 1.0)".to_string()),
                Some("rgba(177, 98, 134, 1.0)".to_string()),
                Some("rgba(104, 157, 106, 1.0)".to_string()),
                Some("rgba(168, 153, 132, 1.0)".to_string()),
                Some("rgba(146, 131, 116, 1.0)".to_string()),
                Some("rgba(251, 73, 52, 1.0)".to_string()),
                Some("rgba(184, 187, 38, 1.0)".to_string()),
                Some("rgba(250, 189, 47, 1.0)".to_string()),
                Some("rgba(131, 165, 152, 1.0)".to_string()),
                Some("rgba(211, 134, 155, 1.0)".to_string()),
                Some("rgba(142, 192, 124, 1.0)".to_string()),
                Some("rgba(235, 219, 178, 1.0)".to_string()),
            ];
        }
        ColorSchemePreset::CatppuccinMocha => {
            settings.background = Some("rgba(30, 30, 46, 1.0)".to_string());
            settings.foreground = Some("rgba(205, 214, 244, 1.0)".to_string());
            settings.palette = vec![
                Some("rgba(73, 77, 100, 1.0)".to_string()),
                Some("rgba(243, 139, 168, 1.0)".to_string()),
                Some("rgba(166, 227, 161, 1.0)".to_string()),
                Some("rgba(249, 226, 175, 1.0)".to_string()),
                Some("rgba(137, 180, 250, 1.0)".to_string()),
                Some("rgba(245, 194, 231, 1.0)".to_string()),
                Some("rgba(148, 226, 213, 1.0)".to_string()),
                Some("rgba(186, 194, 222, 1.0)".to_string()),
                Some("rgba(88, 91, 112, 1.0)".to_string()),
                Some("rgba(243, 139, 168, 1.0)".to_string()),
                Some("rgba(166, 227, 161, 1.0)".to_string()),
                Some("rgba(249, 226, 175, 1.0)".to_string()),
                Some("rgba(137, 180, 250, 1.0)".to_string()),
                Some("rgba(245, 194, 231, 1.0)".to_string()),
                Some("rgba(148, 226, 213, 1.0)".to_string()),
                Some("rgba(166, 173, 200, 1.0)".to_string()),
            ];
        }
    }
    settings
}

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

pub fn load_color_settings() -> ColorSettings {
    let mut settings = ColorSettings::default();
    if let Some(config_path) = get_config_path() {
        if config_path.exists() {
            if let Ok(mut file) = File::open(config_path) {
                let mut contents = String::new();
                if file.read_to_string(&mut contents).is_ok() {
                    let mut preset_from_config: Option<ColorSchemePreset> = None;
                    for line in contents.lines() {
                        let parts: Vec<&str> = line.split('=').map(|s| s.trim()).collect();
                        if parts.len() == 2 {
                            match parts[0] {
                                "active_preset" => {
                                    if let Some(preset) = ColorSchemePreset::from_name(parts[1]) {
                                        settings = get_preset_colors(&preset);
                                        preset_from_config = Some(preset);
                                    }
                                     settings.active_preset = Some(parts[1].to_string());
                                }
                                "foreground" => if preset_from_config.is_none() { settings.foreground = Some(parts[1].to_string()) },
                                "background" => if preset_from_config.is_none() { settings.background = Some(parts[1].to_string()) },
                                key if key.starts_with("color") => {
                                    if preset_from_config.is_none() {
                                        if let Ok(index) = key[5..].parse::<usize>() {
                                            if index < 16 {
                                                settings.palette[index] = Some(parts[1].to_string());
                                            }
                                        }
                                    }
                                }
                                _ => {}
                            }
                        }
                    }
                }
            }
        }
    }
    settings
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
            if let Err(e) = file.write_all(new_lines.join("\\n").as_bytes()) {
                eprintln!("Failed to write to config file: {}", e);
            }
        } else {
            eprintln!("Failed to create or open config file for writing.");
        }
    }
}

pub fn save_color_settings(settings: &ColorSettings) {
    if let Some(config_path) = get_config_path() {
        
        if let Some(parent_dir) = config_path.parent() {
            if !parent_dir.exists() {
                if let Err(e) = fs::create_dir_all(parent_dir) {
                    eprintln!("Failed to create config directory: {}", e);
                    return;
                }
            }
        }

        let mut existing_lines: Vec<String> = Vec::new();
        if config_path.exists() {
            if let Ok(mut file) = File::open(&config_path) {
                let mut contents = String::new();
                if file.read_to_string(&mut contents).is_ok() {
                    existing_lines = contents.lines().map(|s| s.to_string()).collect();
                } else {
                    eprintln!("Failed to read existing config file, will overwrite relevant parts.");
                }
            }
        }

        let mut new_lines: Vec<String> = existing_lines
            .into_iter()
            .filter(|line| {
                !line.starts_with("foreground =") &&
                !line.starts_with("background =") &&
                !line.starts_with("active_preset =") && 
                !(line.starts_with("color") && line.contains('=')) 
            })
            .collect();

        if let Some(preset_name) = &settings.active_preset {
            new_lines.push(format!("active_preset = {}", preset_name));
            
            
            
            
            if ColorSchemePreset::from_name(preset_name).is_none() {
                 
                if let Some(fg) = &settings.foreground {
                    new_lines.push(format!("foreground = {}", fg));
                }
                if let Some(bg) = &settings.background {
                    new_lines.push(format!("background = {}", bg));
                }
                for (i, color_opt) in settings.palette.iter().enumerate() {
                    if let Some(color_val) = color_opt {
                        new_lines.push(format!("color{} = {}", i, color_val));
                    }
                }
            }
        } else { 
            if let Some(fg) = &settings.foreground {
                new_lines.push(format!("foreground = {}", fg));
            }
            if let Some(bg) = &settings.background {
                new_lines.push(format!("background = {}", bg));
            }
            for (i, color_opt) in settings.palette.iter().enumerate() {
                if let Some(color_val) = color_opt {
                    new_lines.push(format!("color{} = {}", i, color_val));
                }
            }
        }


        if let Ok(mut file) = File::create(&config_path) {
            if let Err(e) = file.write_all(new_lines.join("\\\\n").as_bytes()) {
                eprintln!("Failed to write to config file: {}", e);
            }
        } else {
            eprintln!("Failed to create or open config file for writing.");
        }
    }
}
