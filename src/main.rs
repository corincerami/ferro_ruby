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

    println!("Your input: {}", input);
    print!("FeRuby > ");
    io::stdout().flush().ok().expect("Could not flush stdout");
  }
}
