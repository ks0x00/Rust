use base::F;
use complex::Complex;

pub const EPSILON: F = 1E-15;

pub fn is_zero(d: F) -> bool {
    -EPSILON < d && d < EPSILON
}

pub fn equal(x: F, y: F) -> bool {
    x - EPSILON < y && y < x + EPSILON
}

pub fn truncated_string(d: F, prec: usize) -> String {
    let neg = d.is_sign_negative();
    let d = if neg { -d } else { d };
    let td = d.trunc() as u32;
    let fd = (d.fract() * (10.0 as F).powi(prec as i32)).round() as u64;
    let fds = format!("{:0>prec$}", fd);
    if fds.len() > prec {
        format!("{}{}", if neg { "-" } else { "" }, td + 1)
    } else {
        let fds = fds.trim_end_matches('0');
        if fds.len() == 0 {
            if td == 0 {
                "0".to_string()
            } else {
                format!("{}{}", if neg { "-" } else { "" }, td)
            }
        } else {
            format!("{}{}.{}", if neg { "-" } else { "" }, td, fds)
        }
    }
}

pub fn reduce(v: &mut Vec<Complex>) {
    let mut len = v.len() - 1;
    while len > 0 && v[len] == 0.0 {
        len -= 1;
    }
    v.resize(len + 1, Complex::ZERO);
}

pub trait Function {
    fn eval(&self, z: &Complex) -> Complex;
    fn eval_vec(&self, v:&Vec<Complex>) -> Vec<Complex>;
    fn diff(&self) -> Self;
}


pub mod complex;
pub mod polynom;
pub mod rational;
pub mod token;
