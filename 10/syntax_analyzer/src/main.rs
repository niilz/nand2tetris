#[macro_use]
extern crate lazy_static;

mod syntax;

use syntax::tokenizer::{ tokenize, get_tokens_in_xml };
use syntax::analyzer::{ analyze_tokens };
use std::prelude::*;
use std::env;
use std::fs;
use std::fs::File;
use std::io::Write;

fn main() -> std::io::Result<()> {

  // Read a File
  let input_file = "../Square/SquareGame.jack";
  let jack_code = fs::read_to_string(input_file)?;
  // Write File
  let mut output_file = File::create("SquareTEST.xml")?;
  // let jack_code_as_xml_tokens = get_tokens_in_xml(&jack_code);

  let tokens = tokenize(&jack_code);
  analyze_tokens(tokens);
  //output_file.write_all(jack_code_as_xml_tokens.as_bytes());
  Ok(())
}