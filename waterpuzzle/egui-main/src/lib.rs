use eframe::egui::Color32;

pub const RED: Color32 = Color32::from_rgb(255, 0, 0); // FF0000 빨강, 선명한 빨강
pub const ORANGE: Color32 = Color32::from_rgb(255, 128, 0); // FF8000 주황, 밝고 경쾌한 주황.
pub const YELLOW: Color32 = Color32::from_rgb(255, 255, 0); // FFFF00 노랑, 순수한 노랑.
pub const LIME_GREEN: Color32 = Color32::from_rgb(128, 255, 0); // 80FF00 연두, 밝은 녹색 계열.
pub const GREEN: Color32 = Color32::from_rgb(0, 192, 0); // 00C000 초록, 표준적인 초록.
pub const TEAL: Color32 = Color32::from_rgb(0, 192, 192); // 00C0C0 청록, 시원하고 차분한 청록.
pub const BLUE: Color32 = Color32::from_rgb(0, 0, 255); // 0000FF 파랑, 선명한 파랑.
pub const MODIFIED_BLUE: Color32 = Color32::from_rgb(64, 64, 255);
pub const INDIGO: Color32 = Color32::from_rgb(75, 0, 150); // 4B0096 남색, 어둡지만 파랑과 구별되는 남색.
pub const MODIFIED_INDIGO: Color32 = Color32::from_rgb(75, 48, 150);
pub const PURPLE: Color32 = Color32::from_rgb(192, 0, 192); // C000C0 보라, 밝고 뚜렷한 보라.
pub const MAGENTA: Color32 = Color32::from_rgb(255, 0, 128); // FF0080 자홍, 강렬한 자주색 계열.
pub const PINK: Color32 = Color32::from_rgb(255, 128, 192); // FF80C0 분홍, 부드러운 분홍.
pub const BROWN: Color32 = Color32::from_rgb(153, 76, 0); // 994C00 갈색,중간 톤의 갈색.
pub const GREY: Color32 = Color32::from_rgb(128, 128, 128); // 808080 회색, 중간 명도의 회색.
pub const BLACK: Color32 = Color32::from_rgb(0, 0, 0); // 000000 검정, 순수한 검정.
pub const WHITE: Color32 = Color32::from_rgb(255, 255, 255); // FFFFFF 흰색,순수한 흰색.

pub const LIGHT_GREY: Color32 = Color32::from_rgb(172, 172, 172);
pub const DARK_GREY: Color32 = Color32::from_rgb(100, 100, 100);

pub const COLORS: &[Color32] = &[
    WHITE,
    RED,
    MODIFIED_BLUE,
    YELLOW,
    GREEN,
    MAGENTA,
    TEAL,
    ORANGE,
    PURPLE,
    LIME_GREEN,
    MODIFIED_INDIGO,
    PINK,
    BROWN,
    GREY,
    BLACK,
];

pub mod gui;
pub mod viewport;