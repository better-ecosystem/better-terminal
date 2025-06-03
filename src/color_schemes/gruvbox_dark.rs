use crate::config::ColorSettings;

pub fn get_colors() -> ColorSettings {
    ColorSettings {
        foreground: Some("#ebdbb2".to_string()), // fg
        background: Some("#282828".to_string()), // bg
        background_opacity: Some(1.0),
        palette: vec![
            Some("#282828".to_string()), // Normal Black (bg0_h)
            Some("#cc241d".to_string()), // Normal Red
            Some("#98971a".to_string()), // Normal Green
            Some("#d79921".to_string()), // Normal Yellow
            Some("#458588".to_string()), // Normal Blue
            Some("#b16286".to_string()), // Normal Magenta
            Some("#689d6a".to_string()), // Normal Cyan
            Some("#a89984".to_string()), // Normal White (fg4)
            Some("#928374".to_string()), // Bright Black (gray)
            Some("#fb4934".to_string()), // Bright Red
            Some("#b8bb26".to_string()), // Bright Green
            Some("#fabd2f".to_string()), // Bright Yellow
            Some("#83a598".to_string()), // Bright Blue
            Some("#d3869b".to_string()), // Bright Magenta
            Some("#8ec07c".to_string()), // Bright Cyan
            Some("#ebdbb2".to_string()), // Bright White (fg)
        ],
        active_preset: Some("GruvboxDark".to_string()), // This will be overwritten by config::get_preset_colors, but good for consistency
    }
}
