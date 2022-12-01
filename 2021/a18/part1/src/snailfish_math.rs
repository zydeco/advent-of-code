use std::ops::{Add,AddAssign};
use std::ptr;

#[derive(Debug, Clone)]
#[non_exhaustive]
pub enum Number {
    Digit(u8),
    Pair(Box<Number>, Box<Number>),
}

impl std::fmt::Display for Number {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        match self {
            Number::Digit(n) => write!(f, "{}", n)?,
            Number::Pair(a,b) => write!(f, "[{},{}]", *a, *b)?
        }
        Ok(())
    }
}

impl Number {
    pub fn from_str(s: &str) -> Option<Self> {
        let mut chars = s.chars();
        Number::from_chars(&mut chars)
    }

    pub fn from(value: u8) -> Self {
        match value {
            0..=9 => Number::Digit(value),
            10..=254 => {
                let left = value / 2;
                let right = value - left;
                Number::Pair(Box::new(Number::Digit(left)), Box::new(Number::Digit(right)))
            },
            _ => panic!("Value out of range: {}", value)
        }
    }

    fn from_chars(chars: &mut std::str::Chars) -> Option<Self> {
        match chars.next() {
            Some('[') => {
                let left = Number::from_chars(chars).unwrap();
                assert_eq!(',', chars.next().unwrap());
                let right = Number::from_chars(chars).unwrap();
                assert_eq!(']', chars.next().unwrap());
                Some(Number::Pair(Box::new(left), Box::new(right)))
            }
            Some(d) if d.is_digit(10) => Some(Number::Digit(d.to_digit(10).unwrap() as u8)),
            _ => None
        }
    }

    pub fn magnitude(&self) -> u32 {
        match self {
            Number::Pair(l,r) => 3*l.magnitude() + 2*r.magnitude(),
            Number::Digit(n) => *n as u32
        }
    }

    pub fn reduce(&mut self) {
        loop {
            if let Some((left,right,exploding)) = self.explode(0) {
                let mut leaves = Vec::with_capacity(128);
                self.get_leaves(&mut leaves);
                let explode_index = leaves.iter().position(|i| ptr::eq(*i,exploding)).unwrap();
                if explode_index > 0 {
                    *leaves[explode_index-1] += left;
                }
                if explode_index + 1 < leaves.len() {
                    *leaves[explode_index+1] += right;
                }
            } else if !self.split() {
                break;
            }
        }
    }

    fn get_leaves<'b, 'a: 'b>(&'a mut self, leaves: &'b mut Vec<&'a mut Number>) {
        match self {
            Number::Pair(lhs, rhs) => {
                lhs.get_leaves(leaves);
                rhs.get_leaves(leaves);
            },
            Number::Digit(_) => leaves.push(self)
        }
    }

    fn explode(&mut self, level: usize) -> Option<(u8,u8,*const Number)> {
        if let Number::Pair(l,r) = self {
            if level == 4 {
                let values = (l.magnitude() as u8, r.magnitude() as u8, ptr::addr_of!(*self));
                *self = Number::Digit(0);
                return Some(values)
            } else {
                return l.explode(level+1).or_else(|| r.explode(level+1) );
            }
        }
        None
    }

    fn split(&mut self) -> bool {
        match self {
            Number::Pair(l,r) => {
                l.split() || r.split()
            },
            Number::Digit(n) if *n >= 10 => {
                *self = Number::from(*n);
                true
            },
            _ => false
        }
    }
}

impl Add for Number {
    type Output = Number;

    fn add(self, other: Number) -> Number {
        let mut result = Number::Pair(Box::new(self), Box::new(other));
        result.reduce();
        result
    }
}

impl Add<&Number> for Number {
    type Output = Number;

    fn add(self, other: &Number) -> Number {
        let mut result = Number::Pair(Box::new(self), Box::new(other.clone()));
        result.reduce();
        result
    }
}

impl AddAssign<u8> for Number {
    fn add_assign(&mut self, other: u8) {
        match self {
            Number::Digit(n) => *n += other,
            _ => panic!("Can't add u8 to non-digit")
        }
    }
}

impl Add<&Number> for &Number {
    type Output = Number;

    fn add(self, other: &Number) -> Number {
        let mut result = Number::Pair(Box::new(self.clone()), Box::new(other.clone()));
        result.reduce();
        result
    }
}

impl Default for Number {
    fn default() -> Number {
        Number::from(0)
    }
}
