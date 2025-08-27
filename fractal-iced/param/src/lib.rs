use std::{
    sync::{Arc, Mutex},
    thread::available_parallelism,
};

use base::{ColorType, ColorVar, FractalType, Pair, F};
use func::{rational::Rational, truncated_string, Function};
use iced::{Point, Vector};

#[derive(Clone)]
pub struct NParameter {
    pub f: Rational,
    pub g: Rational,
    pub h: Rational,

    pub thresh: F,

    pub fractal_type: FractalType,

    pub min_x: F,
    pub max_x: F,
    pub min_y: F,
    pub max_y: F,
    pub image_size_x: usize,
    pub image_size_y: usize,
    pub unit_x: F,
    pub unit_y: F,

    pub max_iter: usize,
    pub color_type: ColorType,
    pub color_var: ColorVar,
    pub n_workers: usize,
}

impl PartialEq for NParameter {
    fn eq(&self, other: &Self) -> bool {
        self.equal_color_list(other) && self.equal_iters(other)
    }
}

impl From<&SParameter> for NParameter {
    fn from(sp: &SParameter) -> Self {
        Self {
            fractal_type: sp.fractal_type,
            f: Rational::from(sp.f.as_str()),
            g: Rational::from(sp.g.as_str()),
            h: Rational::from(sp.h.as_str()),
            thresh: sp.thresh.parse::<F>().unwrap(),
            min_x: sp.min_x.parse::<F>().unwrap(),
            max_x: sp.max_x.parse::<F>().unwrap(),
            min_y: sp.min_y.parse::<F>().unwrap(),
            max_y: sp.max_y.parse::<F>().unwrap(),
            unit_x: sp.unit_x.parse::<F>().unwrap(),
            unit_y: sp.unit_y.parse::<F>().unwrap(),
            image_size_x: sp.image_size_x.parse::<usize>().unwrap(),
            image_size_y: sp.image_size_y.parse::<usize>().unwrap(),
            max_iter: sp.max_iter.parse::<usize>().unwrap(),
            color_type: sp.color_type,
            color_var: sp.color_var,
            n_workers: sp.n_workers.parse::<usize>().unwrap(),
        }
    }
}

impl NParameter {
    pub fn pixel_coord_x(&self, x: f32) -> F {
        self.min_x + x as F / self.unit_x
    }
    pub fn pixel_coord_y(&self, y: f32) -> F {
        self.max_y - y as F / self.unit_y
    }
    pub fn pixel_coord(&self, p: &Point) -> Pair<F> {
        Pair {
            x: self.min_x + p.x as F / self.unit_x,
            y: self.max_y - p.y as F / self.unit_y,
        }
    }

    pub fn vector_coord_x(&self, x: f32) -> F {
        x as F / self.unit_x
    }
    pub fn vector_coord_y(&self, y: f32) -> F {
        -y as F / self.unit_y
    }
    pub fn vector_coord(&self, v: &Vector) -> Pair<F> {
        Pair {
            x: v.x as F / self.unit_x,
            y: -v.y as F / self.unit_y,
        }
    }

    pub fn center_to(&self, p: &Point, sparam: &mut SParameter) {
        let Pair { x, y } = self.pixel_coord(p);
        let min_x = x + (self.min_x - self.max_x) / 2.0;
        let max_x = x + (self.max_x - self.min_x) / 2.0;
        let min_y = y + (self.min_y - self.max_y) / 2.0;
        let max_y = y + (self.max_y - self.min_y) / 2.0;
        sparam.set_range(min_x, max_x, min_y, max_y);
    }

    pub fn translate(&self, v: &Vector, sparam: &mut SParameter) {
        let v = self.vector_coord(v);
        sparam.set_range(
            self.min_x - v.x,
            self.max_x - v.x,
            self.min_y - v.y,
            self.max_y - v.y,
        );
    }

    pub fn expand_at_center(&self, factor: F, sparam: &mut SParameter) {
        let unit_x = self.unit_x * factor;
        let unit_y = self.unit_y * factor;
        sparam.set_unit(unit_x, unit_y);

        let rf = 1.0 / factor;
        let min_x = 0.5 * ((1.0 + rf) * self.min_x + (1.0 - rf) * self.max_x);
        let max_x = 0.5 * ((1.0 - rf) * self.min_x + (1.0 + rf) * self.max_x);
        let min_y = 0.5 * ((1.0 + rf) * self.min_y + (1.0 - rf) * self.max_y);
        let max_y = 0.5 * ((1.0 - rf) * self.min_y + (1.0 + rf) * self.max_y);
        sparam.set_range(min_x, max_x, min_y, max_y);
    }

    pub fn expand_at_point(&self, p: &Point, factor: F, sparam: &mut SParameter) {
        let Pair { x, y } = self.pixel_coord(p);

        let unit_x = factor * self.unit_x;
        let unit_y = factor * self.unit_y;
        sparam.set_unit(unit_x, unit_y);

        let min_x = x + (self.min_x - x) / factor;
        let max_x = x + (self.max_x - x) / factor;
        let min_y = y + (self.min_y - y) / factor;
        let max_y = y + (self.max_y - y) / factor;
        sparam.set_range(min_x, max_x, min_y, max_y);
    }

    pub fn center_and_expand(&self, p: &Point, factor: F, sparam: &mut SParameter) {
        let Pair { x, y } = self.pixel_coord(p);
        let unit_x = self.unit_x * factor;
        let unit_y = factor * self.unit_y;
        sparam.set_unit(unit_x, unit_y);

        let rf = 1.0 / factor;
        let min_x0 = x + (self.min_x - self.max_x) / 2.0;
        let max_x0 = x + (self.max_x - self.min_x) / 2.0;
        let min_x = 0.5 * ((1.0 + rf) * min_x0 + (1.0 - rf) * max_x0);
        let max_x = 0.5 * ((1.0 - rf) * min_x0 + (1.0 + rf) * max_x0);

        let min_y0 = y + (self.min_y - self.max_y) / 2.0;
        let max_y0 = y + (self.max_y - self.min_y) / 2.0;
        let min_y = 0.5 * ((1.0 + rf) * min_y0 + (1.0 - rf) * max_y0);
        let max_y = 0.5 * ((1.0 - rf) * min_y0 + (1.0 + rf) * max_y0);
        sparam.set_range(min_x, max_x, min_y, max_y);
    }

    pub fn equal_color_list(&self, other: &Self) -> bool {
        self.color_type == other.color_type
            && self.color_var == other.color_var
            && self.max_iter == other.max_iter
    }

    pub fn equal_iters(&self, other: &Self) -> bool {
        self.f == other.f
            && self.g == other.g
            && self.h == other.h
            && self.fractal_type == other.fractal_type
            && self.thresh == other.thresh
            && self.min_x == other.min_x
            && self.max_x == other.max_x
            && self.min_y == other.min_y
            && self.max_y == other.max_y
            // && self.image_size_x == other.image_size_x
            // && self.image_size_y == other.image_size_y
            && self.unit_x == other.unit_x
            && self.unit_y == other.unit_y
            && self.max_iter == other.max_iter
            // && self.color_type == other.color_type
            && self.n_workers == other.n_workers
    }
}

#[derive(Default)]
pub struct SParameter {
    f: String,
    pub g: String,
    pub h: String,
    pub thresh: String,
    pub fractal_type: FractalType,
    min_x: String,
    max_x: String,
    min_y: String,
    max_y: String,
    pub image_size_x: String,
    pub image_size_y: String,
    unit_x: String,
    unit_y: String,
    pub max_iter: String,
    pub color_type: ColorType,
    pub color_var: ColorVar,
    pub n_workers: String,
}

impl SParameter {
    pub fn new() -> Self {
        let mut sparam = Self::default();
        sparam.reset();
        sparam.n_workers = format!(
            "{}",
            match available_parallelism() {
                Ok(num) => num.get(),
                Err(_) => 1,
            }
        );
        sparam
    }
    pub fn check(&self) -> bool {
        if !Rational::check(&self.f, 'z')
            || !Rational::check(&self.g, 'z')
            || !Rational::check(&self.h, 'z')
        {
            return false;
        }
        self.thresh.parse::<F>().is_ok()
            && self.min_x.parse::<F>().is_ok()
            && self.max_x.parse::<F>().is_ok()
            && self.min_y.parse::<F>().is_ok()
            && self.max_y.parse::<F>().is_ok()
            && self.image_size_x.parse::<i32>().is_ok()
            && self.image_size_y.parse::<i32>().is_ok()
            && self.unit_x.parse::<F>().is_ok()
            && self.unit_y.parse::<F>().is_ok()
            && self.max_iter.parse::<usize>().is_ok()
            && self.n_workers.parse::<usize>().is_ok()
    }
    pub fn reset(&mut self) {
        match self.fractal_type {
            FractalType::Mandelbrot => {
                self.f = "z^2".to_string();
                self.h = "z".to_string();
                self.thresh = "2".to_string();
                self.min_x = "-2.1".to_string();
                self.max_x = "1.1".to_string();
                self.min_y = "-1.5".to_string();
                self.max_y = "1.5".to_string();
                self.max_iter = "1000".to_string();
            }
            FractalType::Julia => {
                self.f = "z^2-0.75+0.1234i".to_string();
                self.h = "z".to_string();
                self.thresh = "2".to_string();
                self.min_x = "-2".to_string();
                self.max_x = "2".to_string();
                self.min_y = "-2".to_string();
                self.max_y = "2".to_string();
                self.max_iter = "10000".to_string();
            }
            FractalType::Newton => {
                self.f = "z^3-1".into();
                self.h = "3z^2".to_string();
                self.thresh = "0.000000000001".to_string();
                self.min_x = "-2".to_string();
                self.max_x = "2".to_string();
                self.min_y = "-2".to_string();
                self.max_y = "2".to_string();
                self.max_iter = "100".to_string();
            }
        }
        self.g = "z".to_string();
        self.unit_x = "200".into();
        self.unit_y = "200".to_string();
        self.reset_image_size_x();
        self.reset_image_size_y();
    }

    pub fn image_size_x(&self) -> usize {
        self.image_size_x.parse().unwrap_or(0)
    }
    pub fn image_size_y(&self) -> usize {
        self.image_size_y.parse().unwrap_or(0)
    }

    pub fn reset_image_size_x(&mut self) {
        let min_x = self.min_x.parse().unwrap_or(F::NAN);
        let max_x = self.max_x.parse().unwrap_or(F::NAN);
        let unitx = self.unit_x.parse().unwrap_or(F::NAN);
        if min_x.is_finite() && max_x.is_finite() && unitx.is_finite() {
            self.image_size_x = format!("{}", ((max_x - min_x) * unitx).round() as usize);
        }
    }
    pub fn reset_image_size_y(&mut self) {
        let min_y = self.min_y.parse().unwrap_or(F::NAN);
        let max_y = self.max_y.parse().unwrap_or(F::NAN);
        let unity = self.unit_y.parse().unwrap_or(F::NAN);
        if min_y.is_finite() && max_y.is_finite() && unity.is_finite() {
            self.image_size_y = format!("{}", ((max_y - min_y) * unity).round() as usize);
        }
    }

    pub fn get_f(&self) -> &String {
        &self.f
    }

    pub fn set_f(&mut self, f: String) {
        if self.fractal_type == FractalType::Newton {
            if let Ok(func) = Rational::parse(f.as_str(), 'z') {
                self.h = func.diff().to_string();
            }
        }
        self.f = f;
    }

    pub fn get_min_x(&self) -> &String {
        &self.min_x
    }

    pub fn set_min_x(&mut self, min_x: String) {
        self.min_x = min_x;
        self.reset_image_size_x()
    }
    pub fn get_max_x(&self) -> &String {
        &self.max_x
    }

    pub fn set_max_x(&mut self, max_x: String) {
        self.max_x = max_x;
        self.reset_image_size_x()
    }

    pub fn get_min_y(&self) -> &String {
        &self.min_y
    }

    pub fn set_min_y(&mut self, min_y: String) {
        self.min_y = min_y;
        self.reset_image_size_y()
    }
    pub fn get_max_y(&self) -> &String {
        &self.max_y
    }

    pub fn set_max_y(&mut self, max_y: String) {
        self.max_y = max_y;
        self.reset_image_size_y()
    }

    pub fn get_unit_x(&self) -> &String {
        &self.unit_x
    }

    pub fn set_unit_x(&mut self, unit_x: String) {
        self.unit_x = unit_x;
        self.reset_image_size_x()
    }

    pub fn get_unit_y(&self) -> &String {
        &self.unit_y
    }

    pub fn set_unit_y(&mut self, unit_y: String) {
        self.unit_y = unit_y;
        self.reset_image_size_y()
    }

    pub fn translate(&mut self, tx: F, ty: F) {
        let min_x = self.min_x.parse().unwrap_or(F::NAN);
        let max_x = self.max_x.parse().unwrap_or(F::NAN);
        let min_y = self.min_y.parse().unwrap_or(F::NAN);
        let max_y = self.max_y.parse().unwrap_or(F::NAN);
        if min_x.is_finite() && max_x.is_finite() && min_y.is_finite() && max_y.is_finite() {
            self.min_x = format!("{}", min_x + tx);
            self.max_x = format!("{}", max_x + tx);
            self.min_y = format!("{}", min_y + ty);
            self.max_y = format!("{}", max_y + ty);
        }
    }

    pub fn set_range(&mut self, min_x: F, max_x: F, min_y: F, max_y: F) {
        self.min_x = format!("{min_x}");
        self.max_x = format!("{max_x}");
        self.min_y = format!("{min_y}");
        self.max_y = format!("{max_y}");
        self.reset_image_size_x();
        self.reset_image_size_y();
    }
    pub fn set_unit(&mut self, unit_x: F, unit_y: F) {
        self.unit_x = format!("{unit_x}");
        self.unit_y = format!("{unit_y}");
        self.reset_image_size_x();
        self.reset_image_size_y();
    }

    pub fn mul_unit(&mut self, factor: F) {
        let unit_x = self.unit_x.parse::<F>().unwrap_or(F::NAN);
        let unit_y = self.unit_y.parse::<F>().unwrap_or(F::NAN);
        if unit_x.is_finite() && unit_y.is_finite() {
            self.unit_x = format!("{}", unit_x * factor);
            self.unit_y = format!("{}", unit_y * factor);
            self.reset_image_size_x();
            self.reset_image_size_y();
        }
    }

    pub fn mul_max_iter(&mut self, factor: usize, div: bool) {
        let mut max_iter = match self.max_iter.parse::<usize>() {
            Ok(v) => v,
            Err(_) => return,
        };
        if div {
            max_iter /= factor;
        } else {
            max_iter *= factor;
        }
        self.max_iter = if max_iter == 0 {
            "1".to_string()
        } else {
            format!("{}", max_iter)
        };
    }

    pub fn expand_at_center(&mut self, factor: F) {
        let unit_x = self.unit_x.parse::<F>().unwrap_or(F::NAN);
        if unit_x.is_finite() {
            self.unit_x = format!("{}", unit_x * factor);
        }
        let min_x = self.min_x.parse::<F>().unwrap_or(F::NAN);
        let max_x = self.max_x.parse::<F>().unwrap_or(F::NAN);
        if min_x.is_finite() && max_x.is_finite() {
            self.min_x = format!(
                "{}",
                ((factor + 1.0) * min_x + (factor - 1.0) * max_x) / (2.0 * factor)
            );
            self.max_x = format!(
                "{}",
                ((factor - 1.0) * min_x + (factor + 1.0) * max_x) / (2.0 * factor)
            );
        }
        let unit_y = self.unit_y.parse::<F>().unwrap_or(F::NAN);
        if unit_y.is_finite() {
            self.unit_y = format!("{}", unit_y * factor);
        }
        let min_y = self.min_y.parse::<F>().unwrap_or(F::NAN);
        let max_y = self.max_y.parse::<F>().unwrap_or(F::NAN);
        if min_y.is_finite() && max_y.is_finite() {
            self.min_y = format!(
                "{}",
                ((factor + 1.0) * min_y + (factor - 1.0) * max_y) / (2.0 * factor)
            );
            self.max_y = format!(
                "{}",
                ((factor - 1.0) * min_y + (factor + 1.0) * max_y) / (2.0 * factor)
            );
        }
    }

    pub fn to_text(&self) -> String {
        format!(
            "{};{};{};{};{};{};{};{};{};{}{};{};{};{};{};",
            self.fractal_type.to_string(),
            self.min_x,
            self.max_x,
            self.min_y,
            self.max_y,
            self.unit_x,
            self.unit_y,
            self.max_iter,
            self.n_workers,
            self.color_type.to_string().to_uppercase(),
            self.color_var.to_string(),
            self.f,
            self.g,
            self.h,
            self.thresh
        )
    }

    pub fn from_text(&mut self, text: &str) {
        self.fractal_type = match text.chars().nth(0).unwrap() {
            'M' | 'm' => FractalType::Mandelbrot,
            'J' | 'j' => FractalType::Julia,
            _ => FractalType::Newton,
        };
        let mut rem = text[text.find(';').unwrap() + 1..].to_string();
        let mut pos = rem.find(';').unwrap();
        self.min_x = rem[..pos].trim().to_string();
        rem = rem[pos + 1..].to_string();
        pos = rem.find(';').unwrap();
        self.max_x = rem[..pos].trim().to_string();
        rem = rem[pos + 1..].to_string();
        pos = rem.find(';').unwrap();
        self.min_y = rem[..pos].trim().to_string();
        rem = rem[pos + 1..].to_string();
        pos = rem.find(';').unwrap();
        self.max_y = rem[..pos].trim().to_string();
        rem = rem[pos + 1..].to_string();
        pos = rem.find(';').unwrap();
        self.unit_x = rem[..pos].trim().to_string();
        rem = rem[pos + 1..].to_string();
        pos = rem.find(';').unwrap();
        self.unit_y = rem[..pos].trim().to_string();
        rem = rem[pos + 1..].to_string();
        pos = rem.find(';').unwrap();
        self.max_iter = rem[..pos].trim().to_string();
        rem = rem[pos + 1..].to_string();
        pos = rem.find(';').unwrap();
        self.n_workers = rem[..pos].trim().to_string();
        rem = rem[pos + 1..].to_string();
        pos = rem.find(';').unwrap();
        let ct = rem[..pos].trim();
        self.color_type = ColorType::new(&ct[..3]);
        self.color_var = ColorVar::new(&ct[3..4]);
        rem = rem[pos + 1..].to_string();
        pos = rem.find(';').unwrap();
        self.f = rem[..pos].trim().to_string();
        rem = rem[pos + 1..].to_string();
        pos = rem.find(';').unwrap();
        self.g = rem[..pos].trim().to_string();
        rem = rem[pos + 1..].to_string();
        pos = rem.find(';').unwrap();
        self.h = rem[..pos].trim().to_string();
        rem = rem[pos + 1..].to_string();
        pos = rem.find(';').unwrap();
        self.thresh = rem[..pos].trim().to_string();
        self.reset_image_size_x();
        self.reset_image_size_y()
    }
}

#[derive(Default)]
pub struct DParameter {
    pub x: String,
    pub y: String,
    pub color: String,
    pub iter: String,
    // pub elapsed:String,
    pub elapsed: Arc<Mutex<String>>,
}

impl DParameter {
    pub fn set_xy(&mut self, pos: &Pair<F>) {
        self.x = truncated_string(pos.x, 10);
        self.y = truncated_string(pos.y, 10);
    }

    pub fn set_color(&mut self, rgba: &[u8; 4]) {
        self.color = format!(
            "{:02X}{:02X}{:02X}{:02X}",
            rgba[0], rgba[1], rgba[2], rgba[3]
        );
    }

    pub fn set_iter(&mut self, iter: usize) {
        self.iter = format!("{}", iter);
    }
}
