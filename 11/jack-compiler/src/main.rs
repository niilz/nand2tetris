use jack_compiler::processing::{ process_input };
use std::prelude::*;
use std::env;
use std::path::{ Path };

fn main() {
  // Get path to .jack files from command-line
  let path = env::args().skip(1).next();
  match path {
    Some(path) => process_input(Path::new(&path)),
    None => panic!("please hand me a directory with jack-files"),
  }

  // TOKENIZE INPUT (INTERMEDIATE STEP)
  // Takes pure Jack-code and tokenizes it into XML
  // let jack_code_as_xml_tokens = get_tokens_in_xml(&jack_code);

}
