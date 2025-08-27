use iced::Color;

pub const RED: Color = Color::from_rgb(1.0, 0.0, 0.0); // FF0000 빨강, 선명한 빨강
pub const ORANGE: Color = Color::from_rgb(1.0, 0.5, 0.0); // FF8000 주황, 밝고 경쾌한 주황.
pub const YELLOW: Color = Color::from_rgb(1.0, 1.0, 0.0); // FFFF00 노랑, 순수한 노랑.
pub const LIME_GREEN: Color = Color::from_rgb(0.5, 1.0, 0.0); // 80FF00 연두, 밝은 녹색 계열.
pub const GREEN: Color = Color::from_rgb(0.0, 0.75, 0.0); // 00C000 초록, 표준적인 초록.
pub const TEAL: Color = Color::from_rgb(0.0, 0.75, 0.75); // 00C0C0 청록, 시원하고 차분한 청록.
pub const BLUE: Color = Color::from_rgb(0.0, 0.0, 1.0); // 0000FF 파랑, 선명한 파랑.
pub const MODIFIED_BLUE: Color = Color::from_rgb(0.25, 0.25, 1.0);
pub const INDIGO: Color = Color::from_rgb(0.3, 0.0, 0.6); // 4B0096 남색, 어둡지만 파랑과 구별되는 남색.
pub const MODIFIED_INDIGO: Color = Color::from_rgb(0.3, 0.2, 0.6);
pub const PURPLE: Color = Color::from_rgb(0.75, 0.0, 0.75); // C000C0 보라, 밝고 뚜렷한 보라.
pub const MAGENTA: Color = Color::from_rgb(1.0, 0.0, 0.5); // FF0080 자홍, 강렬한 자주색 계열.
pub const PINK: Color = Color::from_rgb(1.0, 0.5, 0.75); // FF80C0 분홍, 부드러운 분홍.
pub const BROWN: Color = Color::from_rgb(0.6, 0.3, 0.0); // 994C00 갈색,중간 톤의 갈색.
pub const GREY: Color = Color::from_rgb(0.5, 0.5, 0.5); // 808080 회색, 중간 명도의 회색.
pub const BLACK: Color = Color::from_rgb(0.0, 0.0, 0.0); // 000000 검정, 순수한 검정.
pub const WHITE: Color = Color::from_rgb(1.0, 1.0, 1.0); // FFFFFF 흰색,순수한 흰색.

pub const LIGHT_GREY: Color = Color::from_rgb(0.67, 0.67, 0.67);
pub const DARK_GREY: Color = Color::from_rgb(0.4, 0.4, 0.4);

pub const COLORS: &[Color] = &[
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