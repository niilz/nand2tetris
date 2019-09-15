use std::fs::{ File, read_to_string };
use std::env::args;
use std::io::prelude::*;
use std::path::{ Path, PathBuf };

mod translator;
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

    let path = Path::new(input_path);

    let file_stem = path.file_stem().unwrap().to_str().unwrap();

    let output_file = file_stem.to_string() + ".asm";
    let output_path: PathBuf = [path.parent().unwrap().to_str().unwrap(), &output_file].iter().collect();

    // Read input file (specified through command-line)
    let file = read_to_string(input_path);
    let file_as_str = match file {
        Ok(content) => content,
        Err(message) => panic!("File at path '{}‘ could not be read: {}", input_path, message),
    };
    

    // PARSE given file
    let parsed_lines: Vec<Com> = file_as_str
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