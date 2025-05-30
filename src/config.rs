use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::PathBuf;

use crate::color_schemes;

pub const CONFIG_DIR: &str = ".config/better-terminal";
pub const CONFIG_FILE: &str = "better-terminal.conf";

#[derive(Debug, Clone, PartialEq)]
pub enum ColorSchemePreset {
    GruvboxDark,
    CatppuccinMocha,
    Monokai,
    Nord,
    TokyoNight,
    Custom,
}

impl ColorSchemePreset {
    pub fn name(&self) -> &'static str {
        match self {
            ColorSchemePreset::GruvboxDark => "GruvboxDark",
            ColorSchemePreset::CatppuccinMocha => "CatppuccinMocha",
            ColorSchemePreset::Monokai => "Monokai",
            ColorSchemePreset::Nord => "Nord",
            ColorSchemePreset::TokyoNight => "TokyoNight",
            ColorSchemePreset::Custom => "Custom",
        }
    }

    pub fn from_name(name: &str) -> Option<Self> {
        match name {
            "GruvboxDark" => Some(ColorSchemePreset::GruvboxDark),
            "CatppuccinMocha" => Some(ColorSchemePreset::CatppuccinMocha),
            "Monokai" => Some(ColorSchemePreset::Monokai),
            "Nord" => Some(ColorSchemePreset::Nord),
            "TokyoNight" => Some(ColorSchemePreset::TokyoNight),
            "Custom" => Some(ColorSchemePreset::Custom),
            _ => None,
        }
    }

    pub fn all_presets() -> Vec<Self> {
        vec![
            ColorSchemePreset::GruvboxDark,
            ColorSchemePreset::CatppuccinMocha,
            ColorSchemePreset::Monokai,
            ColorSchemePreset::Nord,
            ColorSchemePreset::TokyoNight,
            ColorSchemePreset::Custom,
        ]
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

#[derive(Debug, Clone)]
pub struct AppSettings {
    pub title_bar_visible: bool,
    pub colors: ColorSettings,
}

impl Default for AppSettings {
    fn default() -> Self {
        AppSettings {
            title_bar_visible: true,
            colors: ColorSettings::default(),
        }
    }
}

pub fn get_preset_colors(preset: &ColorSchemePreset) -> ColorSettings {
    let mut settings = ColorSettings::default();
    settings.active_preset = Some(preset.name().to_string());
    match preset {
        ColorSchemePreset::GruvboxDark => {
            settings = color_schemes::gruvbox_dark::get_colors();
        }
        ColorSchemePreset::CatppuccinMocha => {
            settings = color_schemes::catppuccin_mocha::get_colors();
        }
        ColorSchemePreset::Monokai => {
            settings = color_schemes::monokai::get_colors();
        }
        ColorSchemePreset::Nord => {
            settings = color_schemes::nord::get_colors();
        }
        ColorSchemePreset::TokyoNight => {
            settings = color_schemes::tokyo_night::get_colors();
        }
        ColorSchemePreset::Custom => {
            settings = color_schemes::custom::get_colors();
        }
    }
    settings.active_preset = Some(preset.name().to_string());
    settings
}

pub fn get_config_path() -> Option<PathBuf> {
    dirs::home_dir().map(|mut path| {
        path.push(CONFIG_DIR);
        path.push(CONFIG_FILE);
        path
    })
}

pub fn load_app_settings() -> AppSettings {
    let mut app_settings = AppSettings::default();
    if let Some(config_path) = get_config_path() {
        if config_path.exists() {
            if let Ok(mut file) = File::open(config_path) {
                let mut contents = String::new();
                if file.read_to_string(&mut contents).is_ok() {
                    let mut preset_from_config: Option<ColorSchemePreset> = None;

                    for line in contents.lines() {
                        let trimmed_line = line.trim();
                        if trimmed_line.is_empty() {
                            continue;
                        }

                        let parts: Vec<&str> = trimmed_line.split('=').map(|s| s.trim()).collect();
                        if parts.len() == 2 {
                            match parts[0] {
                                "titlebar" => {
                                    app_settings.title_bar_visible = parts[1] == "true";
                                }
                                "active_preset" => {
                                    if let Some(preset) = ColorSchemePreset::from_name(parts[1]) {
                                        app_settings.colors = get_preset_colors(&preset);
                                        preset_from_config = Some(preset);
                                    }
                                    app_settings.colors.active_preset = Some(parts[1].to_string());
                                }
                                "foreground" => if preset_from_config.is_none() {
                                    app_settings.colors.foreground = Some(parts[1].to_string());
                                },
                                "background" => if preset_from_config.is_none() {
                                    app_settings.colors.background = Some(parts[1].to_string());
                                },
                                key if key.starts_with("color") => {
                                    if preset_from_config.is_none() {
                                        if let Ok(index) = key["color".len()..].parse::<usize>() {
                                            if index < app_settings.colors.palette.len() {
                                                app_settings.colors.palette[index] = Some(parts[1].to_string());
                                            }
                                        }
                                    }
                                }
                                _ => {} // Ignore other keys
                            }
                        }
                    }
                }
            }
        }
    }
    app_settings
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
                        let trimmed_line = line.trim();
                        if trimmed_line.is_empty() {
                            continue;
                        }
                        let parts: Vec<&str> = trimmed_line.split('=').map(|s| s.trim()).collect();
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
                                        if let Ok(index) = key["color".len()..].parse::<usize>() {
                                            if index < settings.palette.len() {
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
        
        let mut existing_content = String::new();
        if config_path.exists() {
            if let Ok(mut file) = File::open(&config_path) {
                if file.read_to_string(&mut existing_content).is_err() {
                    eprintln!("Failed to read existing config file, will create/overwrite.");
                    existing_content.clear();
                }
            }
        }

        let normalized_content = existing_content.replace("\\n", "\n");

        let mut output_lines: Vec<String> = normalized_content
            .lines()
            .filter(|line_str| !line_str.trim().starts_with("titlebar ="))
            .map(|s| s.to_string())
            .collect();

        output_lines.push(format!("titlebar = {}", is_visible));

        if let Ok(mut file) = File::create(config_path) {
            if let Err(e) = file.write_all(output_lines.join("\n").as_bytes()) {
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

        let mut existing_content = String::new();
        if config_path.exists() {
            if let Ok(mut file) = File::open(&config_path) {
                if file.read_to_string(&mut existing_content).is_err() {
                    eprintln!("Failed to read existing config file, will create/overwrite relevant parts.");
                    existing_content.clear();
                }
            }
        }
        
        let normalized_content = existing_content.replace("\\n", "\n");

        let mut new_lines: Vec<String> = normalized_content
            .lines()
            .map(|s| s.to_string())
            .filter(|line_str| {
                let trimmed_line = line_str.trim();
                if trimmed_line.starts_with("foreground =") { return false; }
                if trimmed_line.starts_with("background =") { return false; }
                if trimmed_line.starts_with("active_preset =") { return false; }
                if trimmed_line.starts_with("color") && trimmed_line.contains('=') { return false; }
                true 
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
            if let Err(e) = file.write_all(new_lines.join("\n").as_bytes()) {
                eprintln!("Failed to write to config file: {}", e);
            }
        } else {
            eprintln!("Failed to create or open config file for writing.");
        }
    }
}
