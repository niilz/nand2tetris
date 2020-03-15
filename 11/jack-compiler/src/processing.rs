use crate::tokenizer::{ tokenize };
use crate::compiler::{ Compiler };
use std::fs;
use std::io::prelude::*;
use std::path::{ Path, PathBuf };
use std::ffi::OsStr;


pub fn process_input(path: &Path) {
    let dir = fs::read_dir(path).expect("No Path has been passed");
    for item in dir {
      let item = item.expect("no item in path");
      let item_path = item.path();
      if item_path.is_dir() {
        process_input(&item_path);
      } else if item_path.extension() == Some(OsStr::new("jack")) {
        let result_dir = item_path.parent().unwrap();
        parse_jack_file(&item_path, result_dir);
      }
    }
  }


  fn parse_jack_file(jack_file: &Path, result_dir: &Path) {
    let file_stem = jack_file.file_stem().expect("could not read the file stem of the input file");
    // Read a File
    let jack_code = fs::read_to_string(jack_file).expect("could not read file");
    
    // Tokenize code in file
    let tokens = tokenize(&jack_code);
    println!("The code from the file has been tokenized.");
    // Parse tokenized code
    let mut compiler = Compiler::new(&tokens, file_stem.to_str().unwrap());
    let parsed_input = compiler.analyze_tokens();
    println!("The tokens have been analyzed and parsed.");
    // Format xml-data (add line breaks)
    let parsed_and_newline_seperated = seperate_with_newline(parsed_input);
    
    // Write xml to file
    let mut output_file = PathBuf::from(result_dir);
    output_file.push(file_stem); //.to_str().unwrap().to_string() + "niilz");
    output_file.set_extension("vm");
    let mut output_file =  fs::File::create(output_file).expect("Could not create file");
    output_file.write_all(parsed_and_newline_seperated.as_bytes()).expect("could not write to file");
  }


  // Seperate xml elements with New Line
fn seperate_with_newline(commands: Vec<String>) -> String {
    commands.join("\n")
}


// Tests
#[test]
fn new_line_seperation_works() {
  let dummy_tokens = tokenize("class Test {}");
  let mut compiler = Compiler::new(&dummy_tokens, "Test");
  let dummy_parsed_code = compiler.analyze_tokens();
  let dummy_xml_sererated = String::from("TODO Concat with n");
  assert_eq!(seperate_with_newline(dummy_parsed_code), dummy_xml_sererated);

}