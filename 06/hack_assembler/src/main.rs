extern crate hack_assembler;

use hack_assembler::tables::*;
use hack_assembler::encoder::{translate_instruction, get_bin_value};
use hack_assembler::parser::{clean_line, get_command_fields}
;
use std::collections::HashMap;
use std::fs::{self, File};
use std::io::prelude::*;
use std::process;
use std::env;

fn main() {

    // Get's input file and name of output file from user
    // via command-line arguments.
    fn read_command_line() -> (String, String) {
        let args: Vec<_> = env::args().collect();
        if args.len() < 3 {
            println!("Please enter a source file and a destination file.
EXAMPLE: 'cargo run source.txt dest.hack'");
            process::exit(1);
        }
        (args[1].to_string(), args[2].to_string())
    }


    // The work-horse of the program
    // Builds and writes the final binary-file.
    fn build_pure_file<'a>() {

        let (source_file, dest_file) = read_command_line();
        let asm_file_string = fs::read_to_string(source_file).unwrap();

        // Brings the SmybolTable with all predefined (HACK-specified)
        // symbols into scope.
        let mut symbols = SymbolsTable{symbols: HashMap::new()};
        symbols.load_predefined_symbols();

        // First-pass:
        // Goes through every line of the input file and keeps
        // relevant instruction (new-/empty-lines and comments are ignored).
        // Also marks every instruction with the correct CommandType::Variant.
        let mut first_pass = Vec::new();
        for line in asm_file_string.split("\n") {
            let cleaned = clean_line(line);
            if !cleaned.is_empty() {
                let command = get_command_fields(&cleaned);
                let binary_or_label = translate_instruction(command);
                first_pass.push(binary_or_label)
            }
        }
        
        // Second-pass:
        // The counter keeps track of the following line-instruction. Because
        // thats the value a label should have if it's not yet in the symbol-table.
        let mut line_count = 0u32;
        let second_pass: Vec<String> = first_pass.iter().fold(Vec::new(), |mut acc, inst| {
            match &inst[0..1] {
                "#" => {
                    symbols.symbols.entry(&inst[1..]).or_insert(line_count);
                    acc
                },
                _ => {
                    line_count += 1;
                    acc.push(inst.to_string());
                    acc
                },
            }
        });


        // Third-pass
        // Finally resolve all variables. Labels have been dealt with the pass before.
        let mut address_16_plus = 16;
        let third_pass: Vec<String> = second_pass.iter().map(|inst| {
            match &inst[0..1] {
                "*" => {
                    if symbols.symbols.contains_key(&inst[1..]) {
                        let a_inst = format!("{}", symbols.symbols[&inst[1..]]);
                        get_bin_value(&a_inst)
                    } else {
                        symbols.symbols.insert(&inst[1..], address_16_plus);
                        address_16_plus += 1;
                        let symbol = format!("{}", symbols.symbols[&inst[1..]]);
                        get_bin_value(&symbol)
                    }
                },
                _ => inst.to_string(),
            }
        }).collect();

        // Create the ouput file.
        let mut hack_file = match File::create(dest_file) {
            Err(m) => panic!("Could not create file because: {}", m),
            Ok(file) => file,
        };

        // Concat all binary values with a new-line.
        let file_content_str = third_pass.join("\n");

        // Write the final binray-new-line-string to the file.
        match hack_file.write_all(file_content_str.as_bytes()) {
            Err(m) => panic!("Coulnd not write to file because: {}", m),
            Ok(_) => println!("HackFile has been created successfully."),
        }
    }
    
    // Start the readFile-translation-write-file process.
    build_pure_file();
}