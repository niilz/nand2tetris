#[macro_use]
extern crate lazy_static;

mod syntax;

use syntax::tokenizer::{ tokenize };
use syntax::analyzer::{ analyze_tokens };
use std::prelude::*;
use std::ffi::OsStr;
use std::env;
use std::path::{ Path, PathBuf };
use std::fs;
use std::io::Write;

fn main() {
  // Get path to .jack files from command-line
  let path = env::args().skip(1).next();
  match path {
    Some(path) => process_input(Path::new(&path)),
    None => panic!("please hand me a directory with jack-files"),
  }

  fn process_input(path: &Path) {
    let dir = fs::read_dir(path).expect("Where is the path");
    for item in dir {
      let item = item.expect("no item in path");
      let item_path = item.path();
      if item_path.is_dir() {
        process_input(&item_path);
      } else if item_path.extension() == Some(OsStr::new("jack")) {
        parse_jack_file(&item_path);
      }
    }
  }

  fn parse_jack_file(jack_file: &Path) {
    // Read a File
    let jack_code = fs::read_to_string(jack_file).expect("could not read file");
    println!("Jack code from file '{:?}' has been read.", jack_file);
    
    // Tokenize code in file
    let tokens = tokenize(&jack_code);
    println!("The code from the file has been tokenized.");
    // Parse tokenized code
    let parsed_input = analyze_tokens(tokens);
    println!("The tokens have been analyzed and parsed.");
    // Format xml-data (add line breaks)
    let parsed_and_newline_seperated = seperate_with_newline(&parsed_input);
    
    // Write xml to file
    let file_stem = jack_file.file_stem().expect("could not read the file stem of the input file");
    let mut output_file = PathBuf::from(file_stem);
    output_file.set_extension("xml");
    let mut output_file =  fs::File::create(output_file).expect("Could not create file");
    output_file.write_all(parsed_and_newline_seperated.as_bytes()).expect("could not write to file");
    println!("The parsed tokens have been written to file '{:?}'", output_file);
  }

  // TOKENIZE INPUT (INTERMEDIATE STEP)
  // Takes pure Jack-code and tokenizes it into XML
  // let jack_code_as_xml_tokens = get_tokens_in_xml(&jack_code);

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
