#![allow(unused_imports)]
use mathses::{add, divide, MathOperator, modulo, multiply, subtract};
use crate::input::{take_input, take_input_parse};

mod input;

fn main() {
    println!("Welcome to this horrible calculator!");
    let operator: MathOperator = loop {
        let input = take_input_parse("choose an operator: [+-*/]");
        if input.is_ok() {break input.unwrap()};
    };
    let x = take_input_parse("first number?",).unwrap();
    let y = take_input_parse("second number?").unwrap();
    println!("{x} {} {y} = {}", operator, operator.call(x,y));
}

#[test]
fn test_simple() {
    assert_eq!(add(2,2), 4);
    assert_eq!(subtract(120, 40), 80);
    assert_eq!(multiply(44, 3), 132);
    assert_eq!(divide(60, 12), 5);
}

#[test]
fn test_nested() {
    assert_eq!(add(add(add(add(2, 2),2),2),2), 10);
    assert_eq!(multiply(add(divide(subtract(10,4), 3), 9), 4), 44);
    assert_eq!(divide(divide(divide(81, 3), 3), 3), 3);
    assert_eq!(multiply(multiply(multiply(multiply(1, 10), 10), 10), 10), 10_000);
}

#[test]
fn test_modulus() {
    for i in (0..60).step_by(3) {
        assert_eq!(modulo(i, 3), 0);
        assert_eq!(modulo(i+1, 3), 1);
        assert_eq!(modulo(i+2, 3), 2);
    }
}
