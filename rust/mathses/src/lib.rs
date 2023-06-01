use std::fmt::{Display, Formatter};
use std::str::FromStr;

pub fn not_zero(a: i32) -> bool {
    a != 0
}
pub fn is_neg(a: i32) -> bool {
    a < 0
}

pub fn subtract(a:i32, b:i32) -> i32 {
    let mut x = a;
    let mut y = b;
    while not_zero(y) {
        x ^= y;
        y &= x;
        y <<= 1;
    }
    return x;
}
pub fn add(a:i32, b:i32) -> i32 {
    let mut x = a;
    let mut y = b;
    while not_zero(y) {
        let c = x;
        x ^= y;
        y &= c;
        y <<= 1;
    }
    return x;
}

pub fn equal(a:i32, b:i32) -> bool {
    !not_zero(a ^ b)
}
pub fn greater(a:i32, b:i32) -> bool {
    is_neg(subtract(b,a))
}
pub fn less(a:i32, b:i32) -> bool {
    is_neg(subtract(a,b))
}
pub fn greater_equal(a:i32, b:i32) -> bool {
    greater(a,b) || equal(a,b)
}
pub fn less_equal(a:i32, b:i32) -> bool {
    less(a,b) || equal(a,b)
}

pub fn multiply(a:i32, b:i32) -> i32 {
    let mut x = b;
    let mut y = 0;
    while not_zero(x) {
        y = add(y, a);
        x = subtract(x, 1);
    }
    return y;
}
pub fn divide(a:i32, b:i32) -> i32 {
    if !not_zero(b) { return -1; }
    let mut i = 0;
    while less_equal(multiply(i,b),a) {
        i = add(i,1);
    }
    return i-1;
}
pub fn modulo(a:i32, b:i32) -> i32 {
    a - multiply(divide(a,b), b)
}

pub enum MathOperator {
    Addition,
    Subtraction,
    Multiplication,
    Division
}
impl MathOperator {
    pub fn call(&self, x:i32, y:i32) -> i32 {
        match self {
            MathOperator::Addition => add(x,y),
            MathOperator::Subtraction => subtract(x,y),
            MathOperator::Multiplication => multiply(x,y),
            MathOperator::Division => divide(x,y),
        }
    }
}
impl Display for MathOperator {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            MathOperator::Addition => write!(f, "+"),
            MathOperator::Subtraction => write!(f, "-"),
            MathOperator::Multiplication => write!(f, "*"),
            MathOperator::Division => write!(f, "/"),
        }
    }
}
impl FromStr for MathOperator {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "+" => Ok(MathOperator::Addition),
            "-" => Ok(MathOperator::Subtraction),
            "*" => Ok(MathOperator::Multiplication),
            "/" => Ok(MathOperator::Division),
            _ => Err(())
        }
    }
}
