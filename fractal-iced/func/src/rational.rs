use std::{
    fmt::Display,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign},
    vec,
};

use crate::{polynom::Polynomial, Function, token::{self, TokenType, Token}, complex::Complex};

#[derive(Clone)]
pub struct Rational {
    num: Polynomial,
    den: Option<Polynomial>,
}

impl From<f64> for Rational {
    fn from(c: f64) -> Self {
        Self {
            num: Polynomial::from(c),
            den: None,
        }
    }
}
impl From<Complex> for Rational {
    fn from(c: Complex) -> Self {
        Self {
            num: Polynomial::from(c),
            den: None,
        }
    }
}

impl From<Vec<Complex>> for Rational {
    fn from(v: Vec<Complex>) -> Self {
        Self {
            num: Polynomial::from(v),
            den: None,
        }
    }
}

impl From<Polynomial> for Rational {
    fn from(num: Polynomial) -> Self {
        Self { num, den: None }
    }
}

impl From<&str> for Rational {
    fn from(s: &str) -> Self {
        Rational::parse(s, 'z').unwrap()
    }
}

impl Rational {
    pub fn new(num: Polynomial, den: Polynomial) -> Self {
        Rational {
            num,
            den: Some(den),
        }
    }

    pub fn parse(s: &str, var: char) -> Result<Rational, &'static str> {
        let mut tokens = token::to_tokens(s)?;
        if tokens.len() == 0 {
            Err("no equation")
        } else if token::primary_validity(&tokens, var) {
            token::reduce(&mut tokens);
            convert_additives(&tokens)
        } else {
            Err("first validity error")
        }
    }

    pub fn check(s: &str, var: char) -> bool {
        let mut tokens = match token::to_tokens(s) {
            Ok(tokens) => tokens,
            Err(_) => {
                return false;
            }
        };
        if tokens.len() > 0 && token::primary_validity(&tokens, var) {
            token::reduce(&mut tokens);
            match convert_additives(&tokens) {
                Ok(_) => true,
                Err(_) => false,
            }
        } else {
            false
        }
    }

    pub fn num(&self) -> &Polynomial {
        &self.num
    }
    pub fn den(&self) -> Option<&Polynomial> {
        match &self.den {
            Some(den) => Some(den),
            None => None,
        }
    }

    pub fn is_polynomial(&self) -> bool {
        self.den.is_some()
    }

    pub fn reduce(&mut self) {
        self.num.reduce();
        if let Some(den) = &mut self.den {
            den.reduce();
            if den.deg() == 0 {
                self.num /= den.coef()[0];
                self.den = None;
            }
        }
    }
}

impl PartialEq for Rational {
    fn eq(&self, other: &Self) -> bool {
        self.num == other.num && self.den == other.den
    }
}

impl Neg for &Rational {
    type Output = Rational;
    fn neg(self) -> Self::Output {
        Self::Output {
            num: -&self.num,
            den: self.den.clone(),
        }
    }
}

impl AddAssign<&Self> for Rational {
    fn add_assign(&mut self, rhs: &Self) {
        match &mut self.den {
            Some(self_den) => match &rhs.den {
                Some(rhs_den) => {
                    self.num *= rhs_den;
                    self.num += &(&*self_den * &rhs.num);
                    self.num.reduce();
                    *self_den *= rhs_den;
                }
                None => {
                    self.num += &(&*self_den * &rhs.num);
                    self.num.reduce();
                }
            },
            None => match &rhs.den {
                Some(rhs_den) => {
                    self.num *= rhs_den;
                    self.num += &rhs.num;
                    self.num.reduce();
                    self.den = rhs.den.clone();
                }
                None => {
                    self.num += &rhs.num;
                }
            },
        }
    }
}
impl Add for &Rational {
    type Output = Rational;

    fn add(self, rhs: Self) -> Self::Output {
        match &self.den {
            Some(self_den) => match &rhs.den {
                Some(rhs_den) => {
                    let mut num = &self.num * rhs_den;
                    num += &(self_den * &rhs.num);
                    num.reduce();
                    Rational::new(num, self_den * rhs_den)
                }
                None => {
                    let mut num = self_den * &rhs.num;
                    num += &self.num;
                    num.reduce();
                    Rational::new(num, self_den.clone())
                }
            },
            None => match &rhs.den {
                Some(rhs_den) => {
                    let mut num = &self.num * rhs_den;
                    num += &rhs.num;
                    num.reduce();
                    Rational::new(num, rhs_den.clone())
                }
                None => Rational::from(&self.num + &rhs.num),
            },
        }
    }
}

impl SubAssign<&Self> for Rational {
    fn sub_assign(&mut self, rhs: &Self) {
        match &mut self.den {
            Some(self_den) => match &rhs.den {
                Some(rhs_den) => {
                    self.num *= rhs_den;
                    self.num -= &(&*self_den * &rhs.num);
                    self.num.reduce();
                    *self_den *= rhs_den;
                }
                None => {
                    self.num -= &(&*self_den * &rhs.num);
                    self.num.reduce();
                }
            },
            None => match &rhs.den {
                Some(rhs_den) => {
                    self.num *= rhs_den;
                    self.num -= &rhs.num;
                    self.num.reduce();
                    self.den = rhs.den.clone();
                }
                None => {
                    self.num -= &rhs.num;
                }
            },
        }
    }
}
impl Sub for &Rational {
    type Output = Rational;

    fn sub(self, rhs: Self) -> Self::Output {
        match &self.den {
            Some(self_den) => match &rhs.den {
                Some(rhs_den) => {
                    let mut num = &self.num * rhs_den;
                    num -= &(self_den * &rhs.num);
                    num.reduce();
                    Rational::new(num, self_den * rhs_den)
                }
                None => {
                    let mut num = self.num.clone();
                    num -= &(self_den * &rhs.num);
                    num.reduce();
                    Rational::new(num, self_den.clone())
                }
            },
            None => match &rhs.den {
                Some(rhs_den) => {
                    let mut num = &self.num * rhs_den;
                    num -= &rhs.num;
                    num.reduce();
                    Rational::new(num, rhs_den.clone())
                }
                None => Rational::from(&self.num + &rhs.num),
            },
        }
    }
}

impl MulAssign<&Self> for Rational {
    fn mul_assign(&mut self, rhs: &Self) {
        self.num *= &rhs.num;
        if let Some(rhs_den) = &rhs.den {
            match &mut self.den {
                Some(self_den) => {
                    *self_den *= rhs_den;
                }
                None => {
                    self.den = Some(rhs_den.clone());
                }
            }
        }
    }
}

impl Mul for &Rational {
    type Output = Rational;
    fn mul(self, rhs: Self) -> Self::Output {
        let num = &self.num * &rhs.num;
        match &self.den {
            Some(self_den) => match &rhs.den {
                Some(rhs_den) => Self::Output::new(num, self_den * rhs_den),
                None => Self::Output::new(num, self_den.clone()),
            },
            None => match &rhs.den {
                Some(rhs_den) => Self::Output::new(num, rhs_den.clone()),
                None => Self::Output::from(num),
            },
        }
    }
}

impl DivAssign<&Self> for Rational {
    fn div_assign(&mut self, rhs: &Self) {
        if let Some(rhs_den) = &rhs.den {
            self.num *= rhs_den;
        }
        match &mut self.den {
            Some(self_den) => {
                *self_den *= &rhs.num;
            }
            None => {
                self.den = Some(rhs.num.clone());
            }
        }
    }
}

impl Div for &Rational {
    type Output = Rational;
    fn div(self, rhs: Self) -> Self::Output {
        let num = match &rhs.den {
            Some(rhs_den) => &self.num * rhs_den,
            None => self.num.clone(),
        };
        let den = match &self.den {
            Some(self_den) => self_den * &rhs.num,
            None => rhs.num.clone(),
        };
        Self::Output::new(num, den)
    }
}

impl Display for Rational {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.den {
            Some(self_den) => {
                write!(
                    f,
                    "{}/{}",
                    if self.num.is_monomial() {
                        self.num.to_string()
                    } else {
                        format!("({})", self.num)
                    },
                    if self_den.is_single() {
                        self_den.to_string()
                    } else {
                        format!("({})", self_den)
                    }
                )
            }
            None => self.num.fmt(f),
        }
    }
}

impl Function for Rational {
    fn eval(&self, z: &Complex) -> Complex {
        match &self.den {
            Some(den) => self.num.eval(z) / den.eval(z),
            None => self.num.eval(z),
        }
    }

    fn eval_vec(&self, v: &Vec<Complex>) -> Vec<Complex> {
        match &self.den {
            Some(den) => {
                let mut w = self.num.eval_vec(v);
                let d = den.eval_vec(v);
                let mut i = 0;
                while i < w.len() {
                    Complex::div(&mut w[i], &d[i]);
                    i += 1;
                }
                w
            }
            None => self.num.eval_vec(v),
        }
    }

    fn diff(&self) -> Self {
        match &self.den {
            Some(den) => {
                let mut num = self.num.diff();
                num *= den;
                let mut rhs = den.diff();
                rhs *= &self.num;
                num -= &rhs;
                Self {
                    num,
                    den: Some(den * den),
                }
            }
            None => Self::from(self.num.diff()),
        }
    }
}

fn convert_multiplicatives(tokens: &[Box<dyn Token>]) -> Result<Rational, &'static str> {
    let mut p = Rational::from(1.0);
    let len = tokens.len();
    let mut prev: Option<Rational> = None;
    let mut i = 0;
    while i < len {
        match tokens[i].get_type() {
            TokenType::Integer => {
                if let Some(prev_rp) = &prev {
                    p *= prev_rp;
                }
                prev = Some(Rational::from(tokens[i].integer().unwrap() as f64));
            }
            TokenType::Float => {
                if let Some(prev_rp) = prev {
                    p *= &prev_rp;
                }
                prev = Some(Rational::from(tokens[i].float().unwrap()));
            }
            TokenType::Alphabet => {
                if let Some(prev_rp) = &prev {
                    p *= prev_rp;
                }
                if "iI".contains(tokens[i].alphabet().unwrap()) {
                    prev = Some(Rational::from(Complex(0.0, 1.0)));
                } else {
                    prev = Some(Rational::from(vec![Complex(0.0, 0.0), Complex(1.0, 0.0)]));
                }
            }
            TokenType::Power => {
                i += 1;
                match &prev {
                    Some(prev_rp) => {
                        if tokens[i].get_type() == TokenType::Integer
                            && tokens[i].integer().unwrap() >= 0
                        {
                            for _ in 0..tokens[i].integer().unwrap() {
                                p *= prev_rp;
                            }
                            prev = None;
                        } else {
                            return Err("unknown power");
                        }
                    }
                    None => {
                        return Err("power of ???");
                    }
                }
            }
            TokenType::OpenPar => {
                if let Some(prev_rp) = &prev {
                    p *= prev_rp;
                }
                let mut depth = 1;
                let mut j = i;
                while depth > 0 {
                    j += 1;
                    if tokens[j].get_type() == TokenType::OpenPar {
                        depth += 1;
                    } else if tokens[j].get_type() == TokenType::ClosePar {
                        depth -= 1;
                    }
                }
                prev = Some(convert_additives(&tokens[i + 1..j])?);
                i = j;
            }
            TokenType::Slash => {
                if let Some(prev_rp) = &prev {
                    p *= prev_rp;
                }
                i += 1;
                let mut j;
                if tokens[i].get_type() == TokenType::OpenPar {
                    let mut depth = 1;
                    j = i;
                    while depth > 0 {
                        j += 1;
                        if tokens[j].get_type() == TokenType::OpenPar {
                            depth += 1;
                        } else if tokens[j].get_type() == TokenType::ClosePar {
                            depth -= 1;
                        }
                    }
                } else {
                    j = i;
                }
                if j + 1 < len && tokens[j + 1].get_type() == TokenType::Power {
                    j += 2;
                }
                p /= &convert_multiplicatives(&tokens[i..=j])?;
                prev = None;
                i = j;
            }
            _ => {}
        }
        i += 1;
    }
    if let Some(prev_rp) = prev {
        p *= &prev_rp;
    }
    return Ok(p);
}

fn convert_additives(tokens: &[Box<dyn Token>]) -> Result<Rational, &'static str> {
    let mut p = Rational::from(0.0);
    let mut depth = 0;
    let mut i = 0;
    let mut part_start = 0;
    while i < tokens.len() {
        match tokens[i].get_type() {
            TokenType::Plus => {
                if depth == 0 {
                    p += &convert_multiplicatives(&tokens[part_start..i])?;
                    part_start = i + 1;
                }
            }
            TokenType::OpenPar => {
                depth += 1;
            }
            TokenType::ClosePar => {
                depth -= 1;
            }
            _ => {}
        }
        i += 1;
    }
    p += &convert_multiplicatives(&tokens[part_start..])?;
    Ok(p)
}
