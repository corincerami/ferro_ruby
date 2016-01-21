extern crate ferro_ruby;

use ferro_ruby::Operator;

#[test]
fn test_plus() {
    let plus_symbol = "+".into();
    let plus = Operator { symbol: plus_symbol };
    let sum = plus.call(1, 2);
    assert_eq!(sum.unwrap(), 3);
}

#[test]
fn test_minus() {
    let minus_symbol = "-".into();
    let minus = Operator { symbol: minus_symbol };
    let diff = minus.call(1, 2);
    assert_eq!(diff.unwrap(), -1);
}
