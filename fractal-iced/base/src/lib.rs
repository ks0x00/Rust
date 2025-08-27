use std::fmt::Display;

pub const DATA_DIR: &str = "data";
pub const DATA_FILE: &str = "fractal.dat";
pub const INDEX_FILE: &str = "index.dat";
pub const END_OF_LINE: &str = "\r\n";

pub type F = f64;

#[derive(Default, Clone, Copy)]
pub struct Pair<T>
where
    T: Default + Copy,
{
    pub x: T,
    pub y: T,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum FractalType {
    Mandelbrot,
    Julia,
    Newton,
}

impl Default for FractalType {
    fn default() -> Self {
        Self::Mandelbrot
    }
}

impl Display for FractalType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                FractalType::Mandelbrot => "Mandelbrot",
                FractalType::Julia => "Julia",
                FractalType::Newton => "Newton",
            }
        )
    }
}

pub const COLOR_TYPE: [&str; 14] = [
    "Rgb", "Gbr", "Brg", "Rbg", "Bgr", "Grb", "Cmy", "Myc", "Ycm", "Cym", "Ymc", "Mcy", "Hsb",
    "Hsl",
];
pub const COLOR_VAR: [&str; 5] = ["v0", "v1", "v2", "v3", "v4"];

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ColorType {
    Rgb,
    Gbr,
    Brg,
    Rbg,
    Bgr,
    Grb,
    Cmy,
    Myc,
    Ycm,
    Cym,
    Ymc,
    Mcy,
    Hsb,
    Hsl,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ColorVar {
    Var0,
    Var1,
    Var2,
    Var3,
    Var4,
}

impl ColorType {
    pub fn new(ct: &str) -> Self {
        let ct = ct.to_uppercase();
        match ct.as_str() {
            "RGB" => Self::Rgb,
            "GBR" => Self::Gbr,
            "BRG" => Self::Brg,
            "RBG" => Self::Rbg,
            "BGR" => Self::Bgr,
            "GRB" => Self::Grb,
            "CMY" => Self::Cmy,
            "MYC" => Self::Myc,
            "YCM" => Self::Ycm,
            "CYM" => Self::Cym,
            "YMC" => Self::Ymc,
            "MCY" => Self::Mcy,
            "HSB" => Self::Hsb,
            _ => Self::Hsl,
        }
    }
}

impl ColorVar {
    pub fn new(var: &str) -> Self {
        match var.chars().last().unwrap() {
            '0' => Self::Var0,
            '1' => Self::Var1,
            '2' => Self::Var2,
            '3' => Self::Var3,
            _ => Self::Var4,
        }
    }
}

impl Default for ColorType {
    fn default() -> Self {
        Self::Rgb
    }
}
impl Default for ColorVar {
    fn default() -> Self {
        Self::Var0
    }
}

impl Display for ColorType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                ColorType::Rgb => "Rgb",
                ColorType::Gbr => "Gbr",
                ColorType::Brg => "Brg",
                ColorType::Rbg => "Rbg",
                ColorType::Bgr => "Bgr",
                ColorType::Grb => "Grb",
                ColorType::Cmy => "Cmy",
                ColorType::Myc => "Myc",
                ColorType::Ycm => "Ycm",
                ColorType::Cym => "Cym",
                ColorType::Ymc => "Ymc",
                ColorType::Mcy => "Mcy",
                ColorType::Hsb => "Hsb",
                ColorType::Hsl => "Hsl",
            }
        )
    }
}

impl Display for ColorVar {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                ColorVar::Var0 => "0",
                ColorVar::Var1 => "1",
                ColorVar::Var2 => "2",
                ColorVar::Var3 => "3",
                ColorVar::Var4 => "4",
            }
        )
    }
}
