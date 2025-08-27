use std::{
    fmt::Display,
    ops::{Add, AddAssign, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign},
};

use crate::{reduce, Function, complex::Complex, equal};

#[derive(Clone)]
pub struct Polynomial {
    coef: Vec<Complex>,
    deg: usize,
}

impl From<f64> for Polynomial {
    fn from(a: f64) -> Self {
        Self::from(vec![Complex(a, 0.0)])
    }
}

impl From<Complex> for Polynomial {
    fn from(a: Complex) -> Self {
        Self::from(vec![a])
    }
}

impl From<Vec<Complex>> for Polynomial {
    fn from(coef: Vec<Complex>) -> Self {
        let deg = coef.len() - 1;
        Self { coef, deg }
    }
}

impl Polynomial {
    const DISPLAY_ASCEND: bool = false;

    pub fn reduce(&mut self) {
        while self.deg > 0 && self.coef[self.deg] == 0.0 {
            self.deg -= 1;
        }
        self.coef.resize(self.deg + 1, Complex::ZERO);
    }

    pub fn coef(&self) -> &Vec<Complex> {
        &self.coef
    }

    pub fn deg(&self) -> usize {
        self.deg
    }

    pub fn is_monomial(&self) -> bool {
        let mut nonzero = 0;
        for z in self.coef.iter() {
            if *z != 0.0 {
                if nonzero == 1 {
                    return false;
                }
                nonzero += 1;
            }
        }
        true
    }

    pub fn is_single(&self) -> bool {
        if self.deg == 0 {
            self.coef[0].is_real() || self.coef[0] == Complex::I
        } else {
            let mut nonzero = 0;
            for z in self.coef.iter() {
                if *z != 0.0 {
                    if nonzero == 1 || *z != 1.0 {
                        return false;
                    }
                    nonzero += 1;
                }
            }
            true
        }
    }

    pub fn neg(&mut self) {
        for c in self.coef.iter_mut() {
            c.neg();
        }
    }
}

impl Neg for &Polynomial {
    type Output = Polynomial;

    fn neg(self) -> Self::Output {
        Self::Output::from(self.coef.iter().map(|&c| -c).collect::<Vec<Complex>>())
    }
}

impl AddAssign<&Self> for Polynomial {
    fn add_assign(&mut self, rhs: &Self) {
        if self.deg >= rhs.deg {
            for i in 0..=rhs.deg {
                self.coef[i] += rhs.coef[i];
            }
        } else {
            for i in 0..=self.deg {
                self.coef[i] += rhs.coef[i];
            }
            let mut i = self.deg;
            self.coef.resize_with(rhs.deg + 1, || {
                i += 1;
                rhs.coef[i]
            });
            self.deg = rhs.deg;
        }
        self.reduce();
    }
}

impl Add for &Polynomial {
    type Output = Polynomial;

    fn add(self, rhs: Self) -> Self::Output {
        let mut coef;
        if self.deg >= rhs.deg {
            coef = self.coef.clone();
            for i in 0..=rhs.deg {
                coef[i] += rhs.coef[i]
            }
        } else {
            coef = rhs.coef.clone();
            for i in 0..=self.deg {
                coef[i] += self.coef[i]
            }
        }
        reduce(&mut coef);
        Polynomial::from(coef)
    }
}

impl SubAssign<&Self> for Polynomial {
    fn sub_assign(&mut self, rhs: &Self) {
        if self.deg >= rhs.deg {
            for i in 0..=rhs.deg {
                self.coef[i] -= rhs.coef[i];
            }
        } else {
            for i in 0..=self.deg {
                self.coef[i] -= rhs.coef[i];
            }
            let mut i = self.deg;
            self.coef.resize_with(rhs.deg + 1, || {
                i += 1;
                -rhs.coef[i]
            });
            self.deg = rhs.deg;
        }
        self.reduce();
    }
}

impl Sub for &Polynomial {
    type Output = Polynomial;

    fn sub(self, rhs: Self) -> Self::Output {
        let mut coef;
        if self.deg >= rhs.deg {
            coef = self.coef.clone();
            for i in 0..=rhs.deg {
                coef[i] -= rhs.coef[i]
            }
        } else {
            coef = rhs.coef.iter().map(|c| -*c).collect();
            for i in 0..=self.deg {
                coef[i] += self.coef[i]
            }
        }
        reduce(&mut coef);
        Polynomial::from(coef)
    }
}

impl MulAssign<&Self> for Polynomial {
    fn mul_assign(&mut self, rhs: &Self) {
        let deg = self.deg + rhs.deg;
        let mut coef = vec![Complex::ZERO; deg + 1];
        for i in 0..=self.deg {
            for j in 0..=rhs.deg {
                coef[i + j] += self.coef[i] * rhs.coef[j];
            }
        }
        self.coef = coef;
        self.deg = deg;
    }
}

impl Mul for &Polynomial {
    type Output = Polynomial;

    fn mul(self, rhs: Self) -> Self::Output {
        let deg = self.deg + rhs.deg;
        let mut coef = vec![Complex::ZERO; deg + 1];
        for i in 0..=self.deg {
            for j in 0..=rhs.deg {
                coef[i + j] += self.coef[i] * rhs.coef[j];
            }
        }
        Polynomial { coef, deg }
    }
}

impl DivAssign<f64> for Polynomial {
    fn div_assign(&mut self, rhs: f64) {
        for c in self.coef.iter_mut() {
            *c /= rhs;
        }
    }
}

impl DivAssign<Complex> for Polynomial {
    fn div_assign(&mut self, rhs: Complex) {
        for c in self.coef.iter_mut() {
            *c /= rhs;
        }
    }
}

impl PartialEq for Polynomial {
    fn eq(&self, other: &Self) -> bool {
        self.coef == other.coef && self.deg == other.deg
    }
}

// return (string, minus first)
fn term_to_string(coef: Complex, deg: usize) -> (String, bool) {
    if coef.is_zero() {
        (String::new(), false)
    } else if deg == 0 {
        (
            coef.to_string(),
            if coef.is_imag() {
                coef.1 < 0.0
            } else {
                coef.0 < 0.0
            },
        )
    } else {
        let mut s = String::new();
        if coef.is_real() {
            if equal(coef.0, -1.0) {
                s.push('-');
            } else if !equal(coef.0, 1.0) {
                s.push_str(coef.0.to_string().as_str());
            }
        } else if coef.is_imag() {
            s.push_str(coef.to_string().as_str());
        } else {
            s.push_str(format!("({})", coef).as_str());
        }
        s.push('z');
        if deg > 1 {
            s.push_str(format!("^{}", deg).as_str());
        }
        (
            s,
            (coef.is_real() && coef.0 < 0.0) || (coef.is_imag() && coef.1 < 0.0),
        )
    }
}

impl Display for Polynomial {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.deg == 0 {
            return write!(f, "{}", self.coef[0]);
        }
        let mut s;
        if Polynomial::DISPLAY_ASCEND {
            s = if self.coef[0] == 0.0 {
                String::new()
            } else {
                self.coef[0].to_string()
            };
            for i in 1..=self.deg {
                let (s1, minus) = term_to_string(self.coef[i], i);
                if s.len() > 0 && s1.len() > 0 && !minus {
                    s.push('+');
                }
                s.push_str(s1.as_str());
            }
        } else {
            s = term_to_string(self.coef[self.deg], self.deg).0;
            for i in (0..self.deg).rev() {
                let (s1, minus) = term_to_string(self.coef[i], i);
                if s.len() > 0 && s1.len() > 0 && !minus {
                    s.push('+');
                }
                s.push_str(s1.as_str());
            }
        }
        write!(f, "{}", s)
    }
}

impl Function for Polynomial {
    fn eval(&self, z: &Complex) -> Complex {
        // iterators have overheads!
        let mut i = self.deg;
        let mut w = self.coef[i];
        while i > 0 {
            i -= 1;
            w.mul_add(z, &self.coef[i]);
        }
        w
    }

    fn eval_vec(&self, v:&Vec<Complex>) -> Vec<Complex> {
        let mut i = self.deg;
        let mut w = vec![self.coef[i]; v.len()];
        while i > 0 {
            i -= 1;
            let mut j = 0;
            while j < v.len() {
                w[j].mul_add(&v[j], &self.coef[i]);
                j += 1;
            }
        }
        w
    }

    fn diff(&self) -> Self {
        if self.deg == 0 {
            Polynomial::from(0.0)
        } else {
            let mut coef = Vec::new();
            for i in 1..=self.deg {
                coef.push(self.coef[i] * (i as f64));
            }
            Self::from(coef)
        }
    }
}
