use std::io::prelude::*;
use std::io;

fn main() {
  print!("FeRuby > ");
  io::stdout().flush().ok().expect("Could not flush stdout");
  loop {
    let mut input = String::new();

    io::stdin().read_line(&mut input)
        .ok()
        .expect("Failed to read line");

    let result = add(parse_ints(input));
    println!("{:?}", result);
    print!("FeRuby > ");
    io::stdout().flush().ok().expect("Could not flush stdout");
  }
}

fn parse_ints(input: String) -> Vec<i32> {
  let words = input.split_whitespace();
  let mut nums = Vec::new();
  for word in words {
    let num = word.parse::<i32>().ok();

    match num {
      Some(x) => nums.insert(0, x),
      None => (),
    }
  };
  nums
}

fn add(args: Vec<i32>) -> i32 {
  let mut sum:i32 = 0;
  for i in args {
    sum += i;
  }
  sum
}

#[test]
fn test_parse_ints() {
  let input = String::from("1 2 3");
  assert_eq!(parse_ints(input), vec![3, 2, 1]);
  let input = String::from("what is this");
  assert_eq!(parse_ints(input), vec![]);
}

#[test]
fn test_add() {
  let vector = vec![1, 2, 3];
  assert_eq!(add(vector), 6);
  let vector = vec![];
  assert_eq!(add(vector), 0);
}
