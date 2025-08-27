use std::{thread::available_parallelism, sync::Arc};

use base::{ColorType, Fractal, F, Pair};
use eframe::epaint::mutex::Mutex;
use func::{rational::Rational, truncated_string};

#[derive(Clone)]
pub struct NParameter {
    pub f: Rational,
    pub g: Rational,
    pub h: Rational,

    pub thresh: F,

    pub fractal: Fractal,

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
    pub n_workers: usize,
}

impl PartialEq for NParameter {
    fn eq(&self, other: &Self) -> bool {
        self.f == other.f
            && self.g == other.g
            && self.h == other.h
            && self.thresh == other.thresh
            && self.fractal == other.fractal
            && self.min_x == other.min_x
            && self.max_x == other.max_x
            && self.min_y == other.min_y
            && self.max_y == other.max_y
            // && self.image_size_x == other.image_size_x
            // && self.image_size_y == other.image_size_y
            && self.unit_x == other.unit_x
            && self.unit_y == other.unit_y
            && self.max_iter == other.max_iter
            && self.color_type == other.color_type
            && self.n_workers == other.n_workers
    }
}

impl From<&SParameter> for NParameter {
    fn from(sp: &SParameter) -> Self {
        Self {
            fractal: sp.fractal,
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
            n_workers: sp.n_workers.parse::<usize>().unwrap(),
        }
    }
}

impl NParameter {
    pub fn coord(&self, img_x: usize, img_y: usize) -> (F, F) {
        (
            self.min_x + img_x as F * (self.max_x - self.min_x) / self.image_size_x as F,
            self.max_y - img_y as F * (self.max_y - self.min_y) / self.image_size_y as F,
        )
    }

    pub fn center_to(&self, x: F, y: F, sparam: &mut SParameter) {
        let min_x = x + (self.min_x - self.max_x) / 2.0;
        let max_x = x + (self.max_x - self.min_x) / 2.0;
        let min_y = y + (self.min_y - self.max_y) / 2.0;
        let max_y = y + (self.max_y - self.min_y) / 2.0;
        sparam.set_range(min_x, max_x, min_y, max_y);
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

    pub fn expand_at_point(&self, x: F, y: F, factor: F, sparam: &mut SParameter) {
        let unit_x = factor * self.unit_x;
        let unit_y = factor * self.unit_y;
        sparam.set_unit(unit_x, unit_y);

        let min_x = x + (self.min_x - x) / factor;
        let max_x = x + (self.max_x - x) / factor;
        let min_y = y + (self.min_y - y) / factor;
        let max_y = y + (self.max_y - y) / factor;
        sparam.set_range(min_x, max_x, min_y, max_y);
    }

    pub fn center_and_expand(&self, x: F, y: F, factor: F, sparam: &mut SParameter) {
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

    pub fn differ_by_color_list(&self, other: &Self) -> bool {
        self.color_type != other.color_type || self.max_iter != other.max_iter
    }

    pub fn differ_by_iters(&self, other: &Self) -> bool {
        self.f != other.f
            || self.g != other.g
            || self.h != other.h
            || self.fractal != other.fractal
            || self.min_x != other.min_x
            || self.max_x != other.max_x
            || self.min_y != other.min_y
            || self.max_y != other.max_y
            // || self.image_size_x != other.image_size_x
            // || self.image_size_y != other.image_size_y
            || self.unit_x != other.unit_x
            || self.unit_y != other.unit_y
            || self.max_iter != other.max_iter
            // || self.color_type != other.color_type
            || self.n_workers != other.n_workers
    }
}

#[derive(Default)]
pub struct SParameter {
    pub f: String,
    pub g: String,
    pub h: String,
    pub thresh: String,
    pub fractal: Fractal,
    pub min_x: String,
    pub max_x: String,
    pub min_y: String,
    pub max_y: String,
    pub image_size_x: String,
    pub image_size_y: String,
    pub unit_x: String,
    pub unit_y: String,
    pub max_iter: String,
    pub color_type: ColorType,
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
        match self.fractal {
            Fractal::Mandelbrot => {
                self.f = "z^2".to_string();
                self.h = "z".to_string();
                self.thresh = "2".to_string();
                self.min_x = "-2.1".to_string();
                self.max_x = "1.1".to_string();
                self.min_y = "-1.5".to_string();
                self.max_y = "1.5".to_string();
                self.max_iter = "1000".to_string();
            }
            Fractal::Julia => {
                self.f = "z^2-0.75+0.1234i".to_string();
                self.h = "z".to_string();
                self.thresh = "2".to_string();
                self.min_x = "-2".to_string();
                self.max_x = "2".to_string();
                self.min_y = "-2".to_string();
                self.max_y = "2".to_string();
                self.max_iter = "10000".to_string();
            }
            Fractal::Newton => {
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

    pub fn reset_image_size_x(&mut self) {
        let minx: F = self.min_x.parse().unwrap_or(F::NAN);
        let maxx: F = self.max_x.parse().unwrap_or(F::NAN);
        let unitx: F = self.unit_x.parse().unwrap_or(F::NAN);
        if minx.is_finite() && maxx.is_finite() && unitx.is_finite() {
            self.image_size_x = format!("{}", ((maxx - minx) * unitx).round() as usize);
        }
    }
    pub fn reset_image_size_y(&mut self) {
        let miny: F = self.min_y.parse().unwrap_or(F::NAN);
        let maxy: F = self.max_y.parse().unwrap_or(F::NAN);
        let unity: F = self.unit_y.parse().unwrap_or(F::NAN);
        if miny.is_finite() && maxy.is_finite() && unity.is_finite() {
            self.image_size_y = format!("{}", ((maxy - miny) * unity).round() as usize);
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
        let mut unit_x = self.unit_x.parse::<F>().unwrap_or(F::NAN);
        if unit_x.is_finite() {
            unit_x *= factor;
            self.unit_x = format!("{unit_x}");
            self.reset_image_size_x();
        }
        let mut unit_y = self.unit_y.parse::<F>().unwrap_or(F::NAN);
        if unit_y.is_finite() {
            unit_y *= factor;
            self.unit_y = format!("{unit_y}");
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
            "{};{};{};{};{};{};{};{};{};{};{};{};{};{};",
            self.fractal.to_string(),
            self.min_x,
            self.max_x,
            self.min_y,
            self.max_y,
            self.unit_x,
            self.unit_y,
            self.max_iter,
            self.n_workers,
            self.color_type.to_string().to_uppercase(),
            self.f,
            self.g,
            self.h,
            self.thresh
        )
    }

    pub fn from_text(&mut self, text: &str) {
        self.fractal = match text.chars().nth(0).unwrap() {
            'M' | 'm' => Fractal::Mandelbrot,
            'J' | 'j' => Fractal::Julia,
            _ => Fractal::Newton,
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
        self.color_type = ColorType::new(&ct[..3], ct[3..4].parse().unwrap());
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
    pub elapsed: Arc<Mutex<String>>,
}

impl DParameter {
    pub fn set_xy(&mut self, pos: &Pair<F>) {
        self.x = truncated_string(pos.x, 10);
        self.y = truncated_string(pos.y, 10);
    }
}
