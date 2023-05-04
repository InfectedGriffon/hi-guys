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