use crate::config::ColorSettings;

pub fn get_colors() -> ColorSettings {
    let mut settings = ColorSettings::default();
    settings.background = Some("rgba(39, 40, 34, 1.0)".to_string());
    settings.foreground = Some("rgba(248, 248, 242, 1.0)".to_string());
    settings.palette = vec![
        Some("rgba(39, 40, 34, 1.0)".to_string()),    // Black
        Some("rgba(249, 38, 114, 1.0)".to_string()),  // Red
        Some("rgba(166, 226, 46, 1.0)".to_string()),  // Green
        Some("rgba(244, 191, 117, 1.0)".to_string()), // Yellow
        Some("rgba(102, 217, 239, 1.0)".to_string()), // Blue
        Some("rgba(174, 129, 255, 1.0)".to_string()), // Magenta
        Some("rgba(161, 239, 228, 1.0)".to_string()), // Cyan
        Some("rgba(248, 248, 242, 1.0)".to_string()), // White
        Some("rgba(117, 113, 94, 1.0)".to_string()),  // Bright Black
        Some("rgba(249, 38, 114, 1.0)".to_string()),  // Bright Red
        Some("rgba(166, 226, 46, 1.0)".to_string()),  // Bright Green
        Some("rgba(244, 191, 117, 1.0)".to_string()), // Bright Yellow
        Some("rgba(102, 217, 239, 1.0)".to_string()), // Bright Blue
        Some("rgba(174, 129, 255, 1.0)".to_string()), // Bright Magenta
        Some("rgba(161, 239, 228, 1.0)".to_string()), // Bright Cyan
        Some("rgba(249, 248, 245, 1.0)".to_string()), // Bright White
    ];
    settings
}
