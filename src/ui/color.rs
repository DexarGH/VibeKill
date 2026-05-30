use egui::Color32;

pub struct Colors;

impl Colors {
    pub const BACKDROP: Color32 = Color32::from_rgb(24, 24, 32);
    pub const BASE: Color32 = Color32::from_rgb(30, 30, 40);
    pub const HIGHLIGHT: Color32 = Color32::from_rgb(50, 50, 70);
    pub const SUBTEXT: Color32 = Color32::from_rgb(180, 180, 180);
    pub const TEXT: Color32 = Color32::from_rgb(255, 255, 255);
    pub const RED: Color32 = Color32::from_rgb(240, 100, 100);
    pub const ORANGE: Color32 = Color32::from_rgb(240, 140, 90);
    pub const YELLOW: Color32 = Color32::from_rgb(240, 200, 120);
    pub const GREEN: Color32 = Color32::from_rgb(160, 240, 130);
    pub const TEAL: Color32 = Color32::from_rgb(80, 200, 200);
    pub const BLUE: Color32 = Color32::from_rgb(100, 150, 240);
    pub const PURPLE: Color32 = Color32::from_rgb(180, 120, 240);

    pub const ACCENT_COLORS: [(&str, Color32); 7] = [
        ("Red", Self::RED),
        ("Orange", Self::ORANGE),
        ("Yellow", Self::YELLOW),
        ("Green", Self::GREEN),
        ("Teal", Self::TEAL),
        ("Blue", Self::BLUE),
        ("Purple", Self::PURPLE),
    ];

    // Glassmorphism / Acrylic palette
    pub const WINDOW_BG: Color32 = Color32::from_rgba_premultiplied(15, 15, 18, 210);
    pub const ACCENT: Color32 = Color32::from_rgb(155, 81, 224);
    pub const CARD_BG: Color32 = Color32::from_rgba_premultiplied(30, 30, 35, 77);
    pub const CARD_HOVER: Color32 = Color32::from_rgba_premultiplied(45, 45, 50, 102);
    pub const CARD_BORDER: Color32 = Color32::from_rgba_premultiplied(255, 255, 255, 13);
    pub const CARD_BORDER_ACTIVE: Color32 = Color32::from_rgb(155, 81, 224);
    pub const TEXT_DISABLED: Color32 = Color32::from_rgb(136, 136, 136);
    pub const TAB_DIVIDER: Color32 = Color32::from_rgba_premultiplied(255, 255, 255, 26);
    pub const TAB_ACTIVE: Color32 = Color32::from_rgb(155, 81, 224);
    pub const BIND_BG: Color32 = Color32::from_rgba_premultiplied(255, 255, 255, 26);
    pub const BIND_TEXT: Color32 = Color32::from_rgb(170, 170, 170);
    pub const CLOSE_HOVER: Color32 = Color32::from_rgb(255, 68, 68);
    pub const SLIDER_BG: Color32 = Color32::from_rgba_premultiplied(255, 255, 255, 26);
    pub const SLIDER_FILL: Color32 = Color32::from_rgb(155, 81, 224);
    pub const CHECKBOX_BORDER: Color32 = Color32::from_rgba_premultiplied(255, 255, 255, 51);
    pub const POPUP_HEADER: Color32 = Color32::from_rgba_premultiplied(255, 255, 255, 26);
}
