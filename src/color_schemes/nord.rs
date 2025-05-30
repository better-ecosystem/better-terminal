use crate::config::ColorSettings;

pub fn get_colors() -> ColorSettings {
    let mut settings = ColorSettings::default();
    settings.background = Some("rgba(46, 52, 64, 1.0)".to_string()); // Nord0
    settings.foreground = Some("rgba(216, 222, 233, 1.0)".to_string()); // Nord4
    settings.palette = vec![
        Some("rgba(59, 66, 82, 1.0)".to_string()),  // Nord1 (Black)
        Some("rgba(191, 97, 106, 1.0)".to_string()), // Nord11 (Red)
        Some("rgba(163, 190, 140, 1.0)".to_string()), // Nord14 (Green)
        Some("rgba(235, 203, 139, 1.0)".to_string()), // Nord13 (Yellow)
        Some("rgba(129, 161, 193, 1.0)".to_string()), // Nord9 (Blue)
        Some("rgba(180, 142, 173, 1.0)".to_string()), // Nord15 (Magenta)
        Some("rgba(136, 192, 208, 1.0)".to_string()), // Nord8 (Cyan)
        Some("rgba(229, 233, 240, 1.0)".to_string()), // Nord5 (White)
        Some("rgba(76, 86, 106, 1.0)".to_string()),  // Nord3 (Bright Black)
        Some("rgba(191, 97, 106, 1.0)".to_string()), // Nord11 (Bright Red)
        Some("rgba(163, 190, 140, 1.0)".to_string()), // Nord14 (Bright Green)
        Some("rgba(235, 203, 139, 1.0)".to_string()), // Nord13 (Bright Yellow)
        Some("rgba(129, 161, 193, 1.0)".to_string()), // Nord9 (Bright Blue)
        Some("rgba(180, 142, 173, 1.0)".to_string()), // Nord15 (Bright Magenta)
        Some("rgba(143, 188, 187, 1.0)".to_string()), // Nord7 (Bright Cyan)
        Some("rgba(236, 239, 244, 1.0)".to_string()), // Nord6 (Bright White)
    ];
    settings
}
