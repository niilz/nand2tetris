#[macro_use]
extern crate lazy_static;

mod syntax;

use syntax::tokenizer::{ tokenize };
use std::prelude::*;
use std::env;
use std::fs;

fn main() {

  // Read a File
  let file_name = "../Square/Main.jack";
  let content = fs::read_to_string(file_name);
  println!("{:?}", content);

  tokenize("let abc = 55;");
}