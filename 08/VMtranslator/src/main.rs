use std::fs::{ File, read_to_string };
use std::env::args;
use std::io::prelude::*;
use std::path::{ Path, PathBuf };

mod translator;
mod arg_handler;
use arg_handler::{ path_builder };
use translator::parser::Com;
use translator::parser::{ parse_line };
use translator::code_writer::{ write_asm };

fn main() {

    // Get file-name from command-line
    let args: Vec<String> = args().collect();
    let input_path = if args.len() == 2 {
            &args[1]
        } else {
            panic!("Please specify input file or folder!")
        };

    // Pass command-line-arg to path_builder to get:
    // a file-name-label (file_stem), an output-path and a list of vm-files.
    // (If input_path is a file (not a dir), the paths Vec only containes one path.)
    let (file_stem, output_path, paths) = path_builder(Path::new(input_path));
  
    // Read input file (specified through command-line)
    let files_as_str = paths.iter().enumerate().fold(String::new(), |file, (i, path)| {
        match read_to_string(path) {
            Ok(content) => file + &content,
            Err(message) => panic!("File at path '{}‘ could not be read: {}", input_path, message),
        }
    });
    

    // PARSE given file
    let parsed_lines: Vec<Com> = files_as_str
                        .split("\n")
                        .map(|line| parse_line(line))
                        .filter(|command| command != &Com::Empty)
                        .collect();

    let asm_result_vec: Vec<String> = parsed_lines
                                        .iter()
                                        .enumerate()
                                        .map(|(line, command)| write_asm(line, command, &file_stem))
                                        .collect();

    // Create the output file.
    let mut asm_file = match File::create(&output_path) {
        Ok(file) => file,
        Err(m) => panic!("Could not create file because: {}", m),
    };

    // Concat all asm-vecs values with a new-line.
    let file_content_str = asm_result_vec.join("\n");

    // Write the final asm-string to the file.
    match asm_file.write_all(file_content_str.as_bytes()) {
        Ok(_) => println!("HackFile with path: '{:?}' has been created successfully.", output_path),
        Err(m) => panic!("Coulnd not write to file because: {}", m),
    }
}