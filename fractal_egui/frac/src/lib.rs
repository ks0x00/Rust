use base::Fractal;
use func::{rational::Rational, Function, complex::Complex};
use param::NParameter;

pub trait ComputeIter {
    fn compute_iter(&self, z: &Complex) -> usize;
}

pub fn iterator(nparam: &NParameter) -> Box<dyn ComputeIter> {
    match nparam.fractal {
        Fractal::Mandelbrot => Box::new(Mandelbrot::new(nparam)),
        Fractal::Julia => Box::new(Julia::new(nparam)),
        Fractal::Newton => Box::new(Newton::new(nparam)),
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
    pub fn new(nparam: &NParameter) -> Self {
        Self {
            f: nparam.f.clone(),
            g: nparam.g.clone(),
            h: nparam.h.clone(),
            max_iter: nparam.max_iter,
            thresh2: nparam.thresh * nparam.thresh,
        }
    }
}

impl ComputeIter for Mandelbrot {
    fn compute_iter(&self, z: &Complex) -> usize {
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
    pub fn new(nparam: &NParameter) -> Self {
        Self {
            f: nparam.f.clone(),
            g: nparam.g.clone(),
            max_iter: nparam.max_iter,
            thresh2: nparam.thresh * nparam.thresh,
        }
    }
}

impl ComputeIter for Julia {
    fn compute_iter(&self, z: &Complex) -> usize {
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
    pub fn new(nparam: &NParameter) -> Self {
        Self {
            f: nparam.f.clone(),
            g: nparam.g.clone(),
            h: nparam.f.diff(),
            max_iter: nparam.max_iter,
            thresh2: nparam.thresh,
        }
    }
}
impl ComputeIter for Newton {
    fn compute_iter(&self, z: &Complex) -> usize {
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

pub mod color;
pub mod calc;