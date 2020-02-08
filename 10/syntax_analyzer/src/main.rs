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
  let input_file = "../ExpressionLessSquare/Main.jack";
  let jack_code = fs::read_to_string(input_file)?;
  println!("Jack code from file '{}' has been read.", input_file);
  // Write File

  // TOKENIZE INPUT (INTERMEDIATE STEP)
  // Takes pure Jack-code and tokenizes it into XML
  // let jack_code_as_xml_tokens = get_tokens_in_xml(&jack_code);

  // PARSER
  let tokens = tokenize(&jack_code);
  println!("The code from the file has been tokenized.");
  let parsed_input = analyze_tokens(tokens);
  println!("The tokens have been analyzed and parsed.");
  
  let mut output_file =  fs::File::create("TEST.xml")?;
  output_file.write_all(parsed_input.as_bytes())?;
  println!("The parsed tokens have been written to file '{:?}'", output_file);
  Ok(())
}
