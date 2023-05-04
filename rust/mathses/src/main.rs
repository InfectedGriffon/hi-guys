use mathses::{add, divide, multiply, subtract};

fn main() {

}

#[test]
fn test_simple() {
    assert_eq!(add(2,2), 4);
    assert_eq!(subtract(120, 40), 80);
    assert_eq!(multiply(44, 3), 132);
    assert_eq!(divide(60, 12), 5);
}