use std::fmt::Display;

pub const DATA_DIR: &str = "data";
pub const DATA_FILE: &str = "fractal.dat";
pub const INDEX_FILE: &str = "index.dat";
pub const END_OF_LINE: &str = "\r\n";

pub type F = f64;

#[derive(Default)]
pub struct Pair<T>
where
    T: Default,
{
    pub x: T,
    pub y: T,
}

#[derive(Clone, Copy, PartialEq)]
pub enum Fractal {
    Mandelbrot,
    Julia,
    Newton,
}

impl Default for Fractal {
    fn default() -> Self {
        Self::Mandelbrot
    }
}

impl Display for Fractal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Fractal::Mandelbrot => "Mandelbrot",
                Fractal::Julia => "Julia",
                Fractal::Newton => "Newton",
            }
        )
    }
}

pub const COLOR_CLASS: [&str; 8] = ["RGB", "GBR", "BRG", "RBG", "BGR", "GRB", "HSB", "HSL"];

#[derive(Clone, Copy, PartialEq)]
pub enum ColorType {
    // Rgb0, Rgb1, Rgb2, Rgb3, Rgb4, Gbr0, Gbr1, Gbr2, Gbr3, Gbr4, Brg0, Brg1, Brg2, Brg3, Brg4,
    // Rbg0, Rbg1, Rbg2, Rbg3, Rbg4, Bgr0, Bgr1, Bgr2, Bgr3, Bgr4, Grb0, Grb1, Grb2, Grb3, Grb4,
    // Hsb0, Hsb1, Hsb2, Hsb3, Hsb4, Hsl0, Hsl1, Hsl2, Hsl3, Hsl4,
    Rgb0,
    Rgb1,
    Rgb2,
    Rgb3,
    Rgb4,
    Gbr0,
    Gbr1,
    Gbr2,
    Gbr3,
    Gbr4,
    Brg0,
    Brg1,
    Brg2,
    Brg3,
    Brg4,
    Rbg0,
    Rbg1,
    Rbg2,
    Rbg3,
    Rbg4,
    Bgr0,
    Bgr1,
    Bgr2,
    Bgr3,
    Bgr4,
    Grb0,
    Grb1,
    Grb2,
    Grb3,
    Grb4,
    Hsb0,
    Hsb1,
    Hsb2,
    Hsb3,
    Hsb4,
    Hsl0,
    Hsl1,
    Hsl2,
    Hsl3,
    Hsl4,
}

impl Default for ColorType {
    fn default() -> Self {
        Self::Rgb0
    }
}

impl Display for ColorType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.class(), self.var())
    }
}

impl ColorType {
    pub fn new(class: &str, var: u32) -> Self {
        match class {
            "BGR" => match var {
                0 => Self::Bgr0,
                1 => Self::Bgr1,
                2 => Self::Bgr2,
                3 => Self::Bgr3,
                _ => Self::Bgr4, // 4
            },
            "GRB" => match var {
                0 => Self::Grb0,
                1 => Self::Grb1,
                2 => Self::Grb2,
                3 => Self::Grb3,
                _ => Self::Grb4, // 4
            },
            "RBG" => match var {
                0 => Self::Rbg0,
                1 => Self::Rbg1,
                2 => Self::Rbg2,
                3 => Self::Rbg3,
                _ => Self::Rbg4, // 4
            },
            "RGB" => match var {
                0 => Self::Rgb0,
                1 => Self::Rgb1,
                2 => Self::Rgb2,
                3 => Self::Rgb3,
                _ => Self::Rgb4, // 4
            },
            "GBR" => match var {
                0 => Self::Gbr0,
                1 => Self::Gbr1,
                2 => Self::Gbr2,
                3 => Self::Gbr3,
                _ => Self::Gbr4, // 4
            },
            "BRG" => match var {
                0 => Self::Brg0,
                1 => Self::Brg1,
                2 => Self::Brg2,
                3 => Self::Brg3,
                _ => Self::Brg4, // 4
            },
            "HSB" => match var {
                0 => Self::Hsb0,
                1 => Self::Hsb1,
                2 => Self::Hsb2,
                3 => Self::Hsb3,
                _ => Self::Hsb4, // 4
            },
            _ => match var {
                // "HSL"
                0 => Self::Hsl0,
                1 => Self::Hsl1,
                2 => Self::Hsl2,
                3 => Self::Hsl3,
                _ => Self::Hsl4, // 4
            },
        }
    }

    pub fn class(&self) -> String {
        match self {
            ColorType::Rgb0
            | ColorType::Rgb1
            | ColorType::Rgb2
            | ColorType::Rgb3
            | ColorType::Rgb4 => "Rgb".to_string(),
            ColorType::Gbr0
            | ColorType::Gbr1
            | ColorType::Gbr2
            | ColorType::Gbr3
            | ColorType::Gbr4 => "Gbr".to_string(),
            ColorType::Brg0
            | ColorType::Brg1
            | ColorType::Brg2
            | ColorType::Brg3
            | ColorType::Brg4 => "Bgr".to_string(),
            ColorType::Rbg0
            | ColorType::Rbg1
            | ColorType::Rbg2
            | ColorType::Rbg3
            | ColorType::Rbg4 => "Rbg".to_string(),
            ColorType::Bgr0
            | ColorType::Bgr1
            | ColorType::Bgr2
            | ColorType::Bgr3
            | ColorType::Bgr4 => "Bgr".to_string(),
            ColorType::Grb0
            | ColorType::Grb1
            | ColorType::Grb2
            | ColorType::Grb3
            | ColorType::Grb4 => "Grb".to_string(),
            ColorType::Hsb0
            | ColorType::Hsb1
            | ColorType::Hsb2
            | ColorType::Hsb3
            | ColorType::Hsb4 => "Hsb".to_string(),
            ColorType::Hsl0
            | ColorType::Hsl1
            | ColorType::Hsl2
            | ColorType::Hsl3
            | ColorType::Hsl4 => "Hsl".to_string(),
        }
    }
    pub fn var(&self) -> u32 {
        match self {
            ColorType::Rgb0
            | ColorType::Gbr0
            | ColorType::Brg0
            | ColorType::Rbg0
            | ColorType::Bgr0
            | ColorType::Grb0
            | ColorType::Hsb0
            | ColorType::Hsl0 => 0,
            ColorType::Rgb1
            | ColorType::Gbr1
            | ColorType::Brg1
            | ColorType::Rbg1
            | ColorType::Bgr1
            | ColorType::Grb1
            | ColorType::Hsb1
            | ColorType::Hsl1 => 1,
            ColorType::Rgb2
            | ColorType::Gbr2
            | ColorType::Brg2
            | ColorType::Rbg2
            | ColorType::Bgr2
            | ColorType::Grb2
            | ColorType::Hsb2
            | ColorType::Hsl2 => 2,
            ColorType::Rgb3
            | ColorType::Gbr3
            | ColorType::Brg3
            | ColorType::Rbg3
            | ColorType::Bgr3
            | ColorType::Grb3
            | ColorType::Hsb3
            | ColorType::Hsl3 => 3,
            ColorType::Rgb4
            | ColorType::Gbr4
            | ColorType::Brg4
            | ColorType::Rbg4
            | ColorType::Bgr4
            | ColorType::Grb4
            | ColorType::Hsb4
            | ColorType::Hsl4 => 4,
        }
    }
}
