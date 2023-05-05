use mathses::{add, divide, multiply, subtract};
use crate::input::{take_input, take_input_parse};

mod input;

fn main() {
    println!("Welcome to this horrible calculator!");
    let operator: (fn(i32, i32) -> i32, &str) = loop {
        let input = take_input("choose an operator: [+-*/]");
        match input.as_ref().map(String::as_str) {
            Ok("+") => break (add, "+"),
            Ok("-") => break (subtract, "-"),
            Ok("*") => break (multiply, "*"),
            Ok("/") => break (divide, "/"),
            _ => print ! ("please "), // try again
        }
    };
    let x = take_input_parse("first number?",).unwrap();
    let y = take_input_parse("second number?").unwrap();
    let result = operator.0(x,y);
    println!("{x} {} {y} = {result}", operator.1);
}

#[test]
fn test_simple() {
    assert_eq!(add(2,2), 4);
    assert_eq!(subtract(120, 40), 80);
    assert_eq!(multiply(44, 3), 132);
    assert_eq!(divide(60, 12), 5);
}