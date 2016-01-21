#![feature(plugin)]
#![plugin(peg_syntax_ext)]
use std::io::prelude::*;
use std::io;

mod ferro_ruby;
pub use ferro_ruby::operator::Operator;

peg_file! parse("ferro_ruby/parse.rustpeg");
pub use parse::{int, operator};

pub fn main() {
  print!("FerroRuby > ");
  io::stdout().flush().ok().expect("Could not flush stdout");
  loop {
    let mut input = String::new();

    io::stdin().read_line(&mut input)
        .ok()
        .expect("Failed to read line");


    println!("Your input: {}", input);
    print!("FerroRuby > ");
    io::stdout().flush().ok().expect("Could not flush stdout");
  }
}
