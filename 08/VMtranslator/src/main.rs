use std::fs::{ File, read_to_string };
use std::env::args;
use std::io::prelude::*;
use std::path::{ Path };

mod translator;
mod arg_handler;
use arg_handler::{ path_builder };
use translator::parser::Com;
use translator::parser::{ parse_line };
use translator::code_writer::{ write_asm, write_bootstrap };

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
    let (output_path, paths) = path_builder(Path::new(input_path));
  
    // Read the files or every file in the directory (specified through command-line)
    // and store the directory-name with the content-string in a tuple.
    let files_as_path_content_tuples: Vec<(&str, String)> = paths.iter().map(|path| {
        match read_to_string(path) {
            Ok(content) => (path.file_name().unwrap().to_str().unwrap(), content),
            Err(message) => panic!("File at path '{}‘ could not be read: {}", input_path, message),
        }
    }).collect();
    

    // Iterates over the tuple(s) of dir-name|content and collects everything in a vector of
    // tuples with dir-name and a Vec of commands (dir, Vec<Commands>).
    let path_with_parsed_lines: Vec<(&str, Vec<Com>)> = files_as_path_content_tuples
                        .iter()
                        .map(|(path, content)| {
                            (*path, content
                                .split("\n")
                                .map(|line| parse_line(line))
                                .filter(|command| command != &Com::Empty)
                                .collect())
                        }).collect();
    
    // Folds all tuples of dir-name|Command-vec into a Vec of ASM-Strings.
    // (The path/dir-name gets passed to write_asm, so that file-specific labels can be created)
    let mut asm_result_vec: Vec<String> = path_with_parsed_lines
                                        .iter()
                                        .fold(Vec::new(), |mut asm_vec, (path, line_vec)| {
                                            asm_vec.push(
                                                line_vec
                                                    .iter()
                                                    .enumerate()
                                                    .map(|(line, command)| write_asm(line, command, path))
                                                    .collect());
                                            asm_vec
                                        });
                                        
    // If there are more than one vm-file, add the bootstrap code.
    // (Insert at idx 0 of the asm-vec)
    if paths.len() > 1 {
        asm_result_vec.insert(0, write_bootstrap());
    }

    // Create the output file.
    let mut asm_file = match File::create(&output_path) {
        Ok(file) => file,
        Err(m) => panic!("Could not create file because: {}", m),
    };

    // Concat all ASM-Strings in the ASM-Vec with "new-line" into one big String.
    let file_content_str = asm_result_vec.join("\n");

    // Write the final ASM-String to the file.
    match asm_file.write_all(file_content_str.as_bytes()) {
        Ok(_) => println!("HackFile with path: '{:?}' has been created successfully.", output_path),
        Err(m) => panic!("Coulnd not write to file because: {}", m),
    }
}