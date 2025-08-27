use base::FractalType;
use func::{complex::Complex, rational::Rational, Function};
use param::NParameter;

pub trait Iterator {
    fn iterate(&self, z: &Complex) -> usize;
}

pub fn iterator(nparam: &NParameter) -> Box<dyn Iterator> {
    match nparam.fractal_type {
        FractalType::Mandelbrot => Box::new(Mandelbrot::from(nparam)),
        FractalType::Julia => Box::new(Julia::from(nparam)),
        FractalType::Newton => Box::new(Newton::from(nparam)),
    }
}

pub struct Mandelbrot {
    f: Rational,
    g: Rational,
    h: Rational,
    max_iter: usize,
    thresh2: f64,
}

impl Mandelbrot {
    pub fn new(f: &Rational, g: &Rational, h: &Rational, max_iter: usize, thresh: f64) -> Self {
        Self {
            f: f.clone(),
            g: g.clone(),
            h: h.clone(),
            max_iter,
            thresh2: thresh * thresh,
        }
    }
}

impl From<&NParameter> for Mandelbrot {
    fn from(nparam: &NParameter) -> Self {
        Self {
            f: nparam.f.clone(),
            g: nparam.g.clone(),
            h: nparam.h.clone(),
            max_iter: nparam.max_iter,
            thresh2: nparam.thresh * nparam.thresh,
        }
    }
}

impl Iterator for Mandelbrot {
    fn iterate(&self, z: &Complex) -> usize {
        let mut u = self.g.eval(z);
        let u0 = self.h.eval(z);
        let mut i = 0;
        while u.sqr_norm() <= self.thresh2 && i < self.max_iter {
            u = self.f.eval(&u);
            u.add(&u0);
            i += 1;
        }
        i
    }
}

pub struct Julia {
    f: Rational,
    g: Rational,
    max_iter: usize,
    thresh2: f64,
}
impl Julia {
    pub fn new(f: &Rational, g: &Rational, max_iter: usize, thresh: f64) -> Self {
        Self {
            f: f.clone(),
            g: g.clone(),
            max_iter,
            thresh2: thresh * thresh,
        }
    }
}
impl From<&NParameter> for Julia {
    fn from(nparam: &NParameter) -> Self {
        Self {
            f: nparam.f.clone(),
            g: nparam.g.clone(),
            max_iter: nparam.max_iter,
            thresh2: nparam.thresh * nparam.thresh,
        }
    }
}

impl Iterator for Julia {
    fn iterate(&self, z: &Complex) -> usize {
        let mut u = self.g.eval(z);
        let mut i = 0;
        while u.sqr_norm() <= self.thresh2 && i < self.max_iter {
            u = self.f.eval(&u);
            i += 1;
        }
        i
    }
}

pub struct Newton {
    f: Rational,
    g: Rational,
    h: Rational,
    max_iter: usize,
    thresh2: f64,
}

impl Newton {
    pub fn new(f: &Rational, g: &Rational, max_iter: usize, thresh: f64) -> Self {
        Self {
            f: f.clone(),
            g: g.clone(),
            h: f.diff(),
            max_iter,
            thresh2: thresh,
        }
    }
}

impl From<&NParameter> for Newton {
    fn from(nparam: &NParameter) -> Self {
        Self {
            f: nparam.f.clone(),
            g: nparam.g.clone(),
            h: nparam.f.diff(),
            max_iter: nparam.max_iter,
            thresh2: nparam.thresh,
        }
    }
}
impl Iterator for Newton {
    fn iterate(&self, z: &Complex) -> usize {
        let mut u = self.g.eval(z);
        let mut n = self.f.eval(&u);
        let mut i = 0;
        while n.sqr_norm() > self.thresh2 && i < self.max_iter {
            n.div(&self.h.eval(&u));
            u.sub(&n);
            n = self.f.eval(&u);
            i += 1;
        }
        i
    }
}

pub mod calc;
pub mod color;
