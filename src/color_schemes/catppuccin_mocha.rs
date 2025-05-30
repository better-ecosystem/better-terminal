use crate::config::ColorSettings;

pub fn get_colors() -> ColorSettings {
    let mut settings = ColorSettings::default();
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
    settings
}
