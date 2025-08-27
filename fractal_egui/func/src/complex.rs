use std::{
    fmt::Display,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign},
};

use base::F;

use crate::{equal, is_zero};

#[derive(Clone, Copy, Debug)]
pub struct Complex(pub F, pub F);

impl From<F> for Complex {
    fn from(x: F) -> Self {
        Self(x, 0.0)
    }
}

impl Complex {
    pub const I: Complex = Self(0.0, 1.0);
    pub const ZERO: Complex = Self(0.0, 0.0);

    pub fn sqr_norm(&self) -> F {
        self.0 * self.0 + self.1 * self.1
    }

    pub fn is_zero(&self) -> bool {
        is_zero(self.0) && is_zero(self.1)
    }

    pub fn is_real(&self) -> bool {
        is_zero(self.1)
    }

    pub fn is_imag(&self) -> bool {
        is_zero(self.0)
    }

    pub fn bounded_by(&self, ix: F, iy: F, ex: F, ey: F) -> bool {
        ix <= self.0 && self.0 <= ex && iy <= self.1 && self.1 <= ey
    }

    pub fn neg(&mut self) {
        self.0 = -self.0;
        self.1 = -self.1;
    }

    pub fn add(&mut self, a: &Self) {
        self.0 += a.0;
        self.1 += a.1;
    }

    pub fn sub(&mut self, a: &Self) {
        self.0 -= a.0;
        self.1 -= a.1;
    }

    pub fn mul(&mut self, a: &Self) {
        let temp = self.0;
        self.0 = temp * a.0 - self.1 * a.1;
        self.1 = temp * a.1 + self.1 * a.0;
    }

    pub fn div(&mut self, a: &Self) {
        let den = a.0 * a.0 + a.1 * a.1;
        let temp = self.0;
        self.0 = (temp * a.0 + self.1 * a.1) / den;
        self.1 = (self.1 * a.0 - temp * a.1) / den;
    }

    pub fn mul_add(&mut self, a: &Self, b: &Self) {
        let temp = self.0;
        self.0 = temp * a.0 - self.1 * a.1 + b.0;
        self.1 = temp * a.1 + self.1 * a.0 + b.1;
    }
}

impl Neg for Complex {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Self(-self.0, -self.1)
    }
}

impl AddAssign<F> for Complex {
    fn add_assign(&mut self, rhs: F) {
        self.0 += rhs;
    }
}
impl AddAssign for Complex {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
        self.1 += rhs.1;
    }
}

impl SubAssign<F> for Complex {
    fn sub_assign(&mut self, rhs: F) {
        self.0 -= rhs;
    }
}
impl SubAssign for Complex {
    fn sub_assign(&mut self, rhs: Self) {
        self.0 -= rhs.0;
        self.1 -= rhs.1;
    }
}

impl MulAssign<F> for Complex {
    fn mul_assign(&mut self, rhs: F) {
        self.0 *= rhs;
        self.1 *= rhs;
    }
}
impl MulAssign for Complex {
    fn mul_assign(&mut self, rhs: Self) {
        let temp = self.0 * rhs.0 - self.1 * rhs.1;
        self.1 = self.0 * rhs.1 + self.1 * rhs.0;
        self.0 = temp;
    }
}

impl DivAssign<F> for Complex {
    fn div_assign(&mut self, rhs: F) {
        self.0 /= rhs;
        self.1 /= rhs;
    }
}
impl DivAssign for Complex {
    fn div_assign(&mut self, rhs: Self) {
        let den = rhs.0 * rhs.0 + rhs.1 * rhs.1;
        let temp = (self.0 * rhs.0 + self.1 * rhs.1) / den;
        self.1 = (self.1 * rhs.0 - self.0 * rhs.1) / den;
        self.0 = temp;
    }
}

impl Add<F> for Complex {
    type Output = Self;

    fn add(self, rhs: F) -> Self::Output {
        Self(self.0 + rhs, self.1)
    }
}
impl Add for Complex {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl Sub<F> for Complex {
    type Output = Self;

    fn sub(self, rhs: F) -> Self::Output {
        Self(self.0 - rhs, self.1)
    }
}
impl Sub for Complex {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0, self.1 - rhs.1)
    }
}

impl Mul<F> for Complex {
    type Output = Self;

    fn mul(self, rhs: F) -> Self::Output {
        Self(self.0 * rhs, self.1 * rhs)
    }
}
impl Mul for Complex {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self(
            self.0 * rhs.0 - self.1 * rhs.1,
            self.0 * rhs.1 + self.1 * rhs.0,
        )
    }
}

impl Div<F> for Complex {
    type Output = Self;

    fn div(self, rhs: F) -> Self::Output {
        Self(self.0 / rhs, self.1 / rhs)
    }
}
impl Div for Complex {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        let den = rhs.0 * rhs.0 + rhs.1 * rhs.1;
        Self(
            (self.0 * rhs.0 + self.1 * rhs.1) / den,
            (self.1 * rhs.0 - self.0 * rhs.1) / den,
        )
    }
}

impl Display for Complex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.is_real() {
            write!(f, "{}", self.0)
        } else if self.is_imag() {
            if equal(self.1, 1.0) {
                write!(f, "I")
            } else if equal(self.1, -1.0) {
                write!(f, "-I")
            } else {
                write!(f, "{}I", self.1)
            }
        } else {
            let mut s = String::from(self.0.to_string());
            if self.1 > 0.0 {
                s.push('+');
                if !equal(self.1, 1.0) {
                    s.push_str(&self.1.to_string())
                }
                s.push('I');
            } else {
                s.push('-');
                if !equal(self.1, -1.0) {
                    s.push_str(&(-self.1).to_string())
                }
                s.push('I');
            }
            write!(f, "{}", s)
        }
    }
}

impl PartialEq<F> for Complex {
    fn eq(&self, other: &F) -> bool {
        equal(self.0, *other) && equal(self.1, 0.0)
    }
}
impl PartialEq for Complex {
    fn eq(&self, other: &Self) -> bool {
        equal(self.0, other.0) && equal(self.1, other.1)
    }
}
