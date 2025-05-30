use crate::config::ColorSettings;

pub fn get_colors() -> ColorSettings {
    let mut settings = ColorSettings::default();
    settings.background = Some("rgba(26, 27, 38, 1.0)".to_string()); // #1a1b26
    settings.foreground = Some("rgba(169, 177, 214, 1.0)".to_string()); // #a9b1d6
    settings.palette = vec![
        Some("rgba(31, 32, 46, 1.0)".to_string()),  // #1f202e (Black)
        Some("rgba(247, 118, 142, 1.0)".to_string()), // #f7768e (Red)
        Some("rgba(158, 206, 106, 1.0)".to_string()), // #9ece6a (Green)
        Some("rgba(224, 175, 104, 1.0)".to_string()), // #e0af68 (Yellow)
        Some("rgba(122, 162, 247, 1.0)".to_string()), // #7aa2f7 (Blue)
        Some("rgba(187, 154, 247, 1.0)".to_string()), // #bb9af7 (Magenta)
        Some("rgba(130, 204, 227, 1.0)".to_string()), // #82cce3 (Cyan)
        Some("rgba(192, 198, 222, 1.0)".to_string()), // #c0c6de (White)
        Some("rgba(68, 73, 92, 1.0)".to_string()),  // #44495c (Bright Black)
        Some("rgba(247, 118, 142, 1.0)".to_string()), // #f7768e (Bright Red)
        Some("rgba(158, 206, 106, 1.0)".to_string()), // #9ece6a (Bright Green)
        Some("rgba(224, 175, 104, 1.0)".to_string()), // #e0af68 (Bright Yellow)
        Some("rgba(122, 162, 247, 1.0)".to_string()), // #7aa2f7 (Bright Blue)
        Some("rgba(187, 154, 247, 1.0)".to_string()), // #bb9af7 (Bright Magenta)
        Some("rgba(130, 204, 227, 1.0)".to_string()), // #82cce3 (Bright Cyan)
        Some("rgba(169, 177, 214, 1.0)".to_string()), // #a9b1d6 (Bright White)
    ];
    settings
}
