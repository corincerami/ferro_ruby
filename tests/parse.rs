extern crate ferro_ruby;

use ferro_ruby::{int, operator, Operator};

#[test]
fn test_int() {
  assert_eq!(int("52"), Ok(52));
  assert_eq!(int("1000"), Ok(1000));
  assert_eq!(int("0"), Ok(0));
  assert!(int("NaN").is_err());
  assert!(int("9223372036854775808").is_err());
  assert_eq!(int("-12"), Ok(-12));
}

#[test]
fn test_operator() {
    let op = Operator { symbol: "+".into() };
    let parsed_op = operator("+").unwrap();
    assert_eq!(parsed_op.symbol, op.symbol);
}
