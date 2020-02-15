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
  let input_file = "../Square/Square.jack";
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
  let parsed_and_newline_seperated = seperate_with_newline(&parsed_input);
  
  let mut output_file =  fs::File::create("TEST.xml")?;
  output_file.write_all(parsed_and_newline_seperated.as_bytes())?;
  println!("The parsed tokens have been written to file '{:?}'", output_file);
  Ok(())
}

// Seperate xml elements with New Line
fn seperate_with_newline(xml: &str) -> String {
  let len = xml.len();
  (0..len)
      .fold(String::new(), |mut res, idx| {
          res.push_str(&xml[idx..idx+1]);
          if idx < len-1 && &xml[idx..idx+1] == ">" && &xml[idx+1..idx+2] == "<" {
              res.push_str("\n");
          }
          res
      }) + "\n"
}

// Tests
#[test]
fn new_line_seperation_works() {
    let dummy_parsed_string = analyze_tokens(tokenize("class Test {}"));
    let dummy_xml_sererated = "<class>\n<keyword> class </keyword>\n<identifier> Test </identifier>\n<symbol> { </symbol>\n<symbol> } </symbol>\n</class>\n";
    assert_eq!(seperate_with_newline(&dummy_parsed_string), dummy_xml_sererated);

}
