#[macro_use]
extern crate lazy_static;

mod syntax;

use syntax::tokenizer::{ get_tokens_in_xml };
use std::prelude::*;
use std::env;
use std::fs;
use std::fs::File;
use std::io::Write;

fn main() -> std::io::Result<()> {

  // Read a File
  let input_file = "../ArrayTest/Main.jack";
  let jack_code = fs::read_to_string(input_file)?;
  // Write File
  let mut output_file = File::create("ArrayMainT.xml")?;
  let jack_code_as_xml_tokens = get_tokens_in_xml(&jack_code);
  output_file.write_all(jack_code_as_xml_tokens.as_bytes());
  Ok(())
}