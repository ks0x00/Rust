use std::fmt::Display;

#[derive(Clone, Copy, PartialEq)]
pub enum TokenType {
    Integer,
    Float,
    Alphabet,
    Plus,
    Minus,
    Star,
    Slash,
    Power,
    OpenPar,
    ClosePar,
    Complex,
}

impl TokenType {
    fn operation(&self) -> bool {
        match *self {
            Self::Plus | Self::Minus | Self::Star | Self::Slash | Self::Power => true,
            _ => false,
        }
    }
}

pub trait Token: Display {
    fn get_type(&self) -> TokenType;

    fn symbol(&self) -> Option<char> {
        None
    }
    fn integer(&self) -> Option<i32> {
        None
    }
    fn float(&self) -> Option<f64> {
        None
    }
    fn alphabet(&self) -> Option<char> {
        None
    }

    fn operation(&self) -> bool {
        self.get_type().operation()
    }
}

pub struct SymbolToken(TokenType);

impl SymbolToken {
    pub fn is_symbol(c: char) -> bool {
        match c {
            '+' | '-' | '*' | '/' | '^' | '(' | ')' => true,
            _ => false,
        }
    }
    pub fn new(c: char) -> Option<Self> {
        match c {
            '+' => Some(Self(TokenType::Plus)),
            '-' => Some(Self(TokenType::Minus)),
            '*' => Some(Self(TokenType::Star)),
            '/' => Some(Self(TokenType::Slash)),
            '^' => Some(Self(TokenType::Power)),
            '(' => Some(Self(TokenType::OpenPar)),
            ')' => Some(Self(TokenType::ClosePar)),
            _ => None,
        }
    }
}

impl Display for SymbolToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Symbol('{}')",
            match self.0 {
                TokenType::Plus => '+',
                TokenType::Minus => '-',
                TokenType::Star => '*',
                TokenType::Slash => '/',
                TokenType::Power => '^',
                TokenType::OpenPar => '(',
                TokenType::ClosePar => ')',
                _ => ' ',
            }
        )
    }
}

impl Token for SymbolToken {
    fn get_type(&self) -> TokenType {
        self.0
    }
    fn symbol(&self) -> Option<char> {
        match self.0 {
            TokenType::Plus => Some('+'),
            TokenType::Minus => Some('-'),
            TokenType::Star => Some('*'),
            TokenType::Slash => Some('/'),
            TokenType::Power => Some('^'),
            TokenType::OpenPar => Some('('),
            TokenType::ClosePar => Some(')'),
            _ => None,
        }
    }
}

pub struct AlphabetToken(char);

impl AlphabetToken {
    pub fn is_alphabet(c: char) -> bool {
        ('A' <= c && c <= 'Z') || ('a' <= c && c <= 'z')
    }

    pub fn new(c: char) -> Option<Self> {
        if ('A' <= c && c <= 'Z') || ('a' <= c && c <= 'z') {
            Some(Self(c))
        } else {
            None
        }
    }
}

impl Token for AlphabetToken {
    fn get_type(&self) -> TokenType {
        TokenType::Alphabet
    }

    fn alphabet(&self) -> Option<char> {
        Some(self.0)
    }
}

impl Display for AlphabetToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "AlphabetToken({})", self.0)
    }
}
pub struct IntegerToken(i32);

impl IntegerToken {
    pub fn new(x: i32) -> Self {
        Self(x)
    }
}

impl Token for IntegerToken {
    fn get_type(&self) -> TokenType {
        TokenType::Integer
    }

    fn integer(&self) -> Option<i32> {
        Some(self.0)
    }
}

impl Display for IntegerToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "IntegerToken({})", self.0)
    }
}

impl PartialEq for IntegerToken {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl PartialOrd for IntegerToken {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.0.partial_cmp(&other.0)
    }
}

pub struct FloatToken(f64);
impl FloatToken {
    pub fn new(x: f64) -> Self {
        Self(x)
    }
}
impl Token for FloatToken {
    fn get_type(&self) -> TokenType {
        TokenType::Float
    }

    fn float(&self) -> Option<f64> {
        Some(self.0)
    }
}

impl Display for FloatToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "FloatToken({})", self.0)
    }
}

pub fn to_tokens(s: &str) -> Result<Vec<Box<dyn Token>>, &'static str> {
    let mut tokens: Vec<Box<dyn Token>> = Vec::new();
    let vs: Vec<char> = s.trim().chars().collect();
    let mut i = 0;
    while i < vs.len() {
        let c = vs[i];
        if let Some(token) = SymbolToken::new(c) {
            tokens.push(Box::new(token));
        } else if let Some(token) = AlphabetToken::new(c) {
            tokens.push(Box::new(token));
        } else if c == '.' || ('0' <= c && c <= '9') {
            let mut dot = c == '.';
            let mut j = i + 1;
            while j < vs.len() {
                if vs[j] == '.' {
                    if dot {
                        break;
                    } else {
                        dot = true;
                    }
                } else if vs[j] < '0' || '9' < vs[j] {
                    break;
                }
                j += 1;
            }
            if dot {
                tokens.push(Box::new(FloatToken::new(s[i..j].parse::<f64>().unwrap())));
            } else {
                tokens.push(Box::new(IntegerToken::new(s[i..j].parse::<i32>().unwrap())));
            }
            i = j - 1;
        } else if c != ' ' {
            return Err("unknown token");
        }
        i += 1;
    }
    Ok(tokens)
}

pub fn primary_validity(tokens: &[Box<dyn Token>], var: char) -> bool {
    let accepted_chars = format!("Ii{}", var);
    let mut prev_opt: Option<TokenType> = None;
    let mut curr_cls;
    let mut depth = 0;
    for i in 0..tokens.len() {
        curr_cls = tokens[i].get_type();
        if curr_cls.operation() {
            if let Some(prev_cls) = prev_opt {
                if prev_cls.operation()
                    || (prev_cls == TokenType::OpenPar
                        && (curr_cls == TokenType::Star || curr_cls == TokenType::Slash))
                {
                    return false;
                }
            }
        } else {
            if curr_cls == TokenType::Alphabet {
                if !accepted_chars.contains(tokens[i].alphabet().unwrap()) {
                    return false;
                }
            } else if curr_cls == TokenType::ClosePar {
                if depth > 0 {
                    depth -= 1;
                } else {
                    return false;
                }
                if let Some(prev_cls) = prev_opt {
                    if prev_cls.operation() || prev_cls == TokenType::OpenPar {
                        return false;
                    }
                }
            } else if curr_cls == TokenType::OpenPar {
                depth += 1;
            }
        }
        prev_opt = Some(curr_cls);
    }
    depth == 0 && !tokens.last().unwrap().operation()
}

pub fn reduce(tokens: &mut Vec<Box<dyn Token>>) {
    let mut prev_opt: Option<TokenType> = None;
    let mut i = 0;
    while i < tokens.len() {
        let curr_cls = tokens[i].get_type();
        if curr_cls == TokenType::Plus {
            match prev_opt {
                Some(prev_cls) => {
                    if prev_cls == TokenType::OpenPar {
                        tokens.remove(i);
                    }
                }
                None => {
                    tokens.remove(i);
                }
            }
        } else if curr_cls == TokenType::Minus {
            tokens.remove(i);
            tokens.insert(i, Box::new(IntegerToken::new(-1)));
            if let Some(prev_cls) = prev_opt {
                if prev_cls != TokenType::OpenPar {
                    tokens.insert(i, Box::new(SymbolToken::new('+').unwrap()));
                    i += 1;
                }
            }
            prev_opt = Some(TokenType::Integer);
        } else if curr_cls == TokenType::Star {
            tokens.remove(i);
            i -= 1;
        } else {
            prev_opt = Some(curr_cls);
        }
        i += 1;
    }
}
