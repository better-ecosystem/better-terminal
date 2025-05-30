use crate::config::ColorSettings;

pub fn get_colors() -> ColorSettings {
    let mut settings = ColorSettings::default();
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
    settings
}
