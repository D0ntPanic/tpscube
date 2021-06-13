use eframe::egui::Color32;

pub enum Theme {
    Background,
    BackgroundDark,
    BackgroundWindow,
    BackgroundHighlight,
    Content,
    Light,
    Disabled,
    Selection,
    SelectionLight,
    Green,
    Red,
    Blue,
    DarkBlue,
    Cyan,
    LightCyan,
    Orange,
    Yellow,
    Magenta,
}

impl Into<Color32> for Theme {
    fn into(self) -> Color32 {
        match self {
            Theme::Background => Color32::from_rgb(44, 44, 44),
            Theme::BackgroundDark => Color32::from_rgb(34, 34, 34),
            Theme::BackgroundWindow => Color32::from_rgb(56, 56, 56),
            Theme::BackgroundHighlight => Color32::from_rgb(64, 64, 64),
            Theme::Content => Color32::from_rgb(224, 224, 224),
            Theme::Light => Color32::from_rgb(120, 120, 120),
            Theme::Disabled => Color32::from_rgb(160, 160, 160),
            Theme::Selection => Color32::from_rgb(96, 96, 96),
            Theme::SelectionLight => Color32::from_rgb(128, 128, 128),
            Theme::Green => Color32::from_rgb(162, 217, 175),
            Theme::Red => Color32::from_rgb(222, 143, 151),
            Theme::Blue => Color32::from_rgb(128, 198, 233),
            Theme::DarkBlue => Color32::from_rgb(86, 121, 139),
            Theme::Cyan => Color32::from_rgb(142, 230, 237),
            Theme::LightCyan => Color32::from_rgb(176, 221, 228),
            Theme::Orange => Color32::from_rgb(237, 189, 129),
            Theme::Yellow => Color32::from_rgb(237, 223, 179),
            Theme::Magenta => Color32::from_rgb(218, 196, 209),
        }
    }
}
