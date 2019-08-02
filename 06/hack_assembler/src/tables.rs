//! Supplies all necessary tables, specified by the HACK-spec.
//!
use std::collections::HashMap;
/// Struct that holds only one HashMap.
/// It get's initialized with `load_predefined_symbols`
/// which loads all base symbols (given by the HACK-specification)
/// into the HashMap.
/// The HashMap inside of a SymbolsTable is mutable so it can be 
/// polpulated with more entries at runtime.
///
#[derive(Debug)]
pub struct SymbolsTable<'a> {
    pub symbols: HashMap<&'a str, u32>,
}
impl<'a> SymbolsTable<'a> {

    /// By calling this function, all predifined (by the HACK-spec) symbols
    /// are filled into the `symbols` `HashMap` of a `SymbolsTable`.
    pub fn load_predefined_symbols(&mut self) {
        self.symbols.insert("R0", 0u32);
        self.symbols.insert("R1", 1u32);
        self.symbols.insert("R2", 2u32);
        self.symbols.insert("R3", 3u32);
        self.symbols.insert("R4", 4u32);
        self.symbols.insert("R5", 5u32);
        self.symbols.insert("R6", 6u32);
        self.symbols.insert("R7", 7u32);
        self.symbols.insert("R8", 8u32);
        self.symbols.insert("R9", 9u32);
        self.symbols.insert("R10", 10u32);
        self.symbols.insert("R11", 11u32);
        self.symbols.insert("R12", 12u32);
        self.symbols.insert("R13", 13u32);
        self.symbols.insert("R14", 14u32);
        self.symbols.insert("R15", 15u32);
        self.symbols.insert("SCREEN", 16384u32);
        self.symbols.insert("KBD", 24576u32);
        self.symbols.insert("SP", 0u32);
        self.symbols.insert("LCL", 1u32);
        self.symbols.insert("ARG", 2u32);
        self.symbols.insert("THIS", 3u32);
        self.symbols.insert("THAT", 4u32);
    }
}

/// The function returns a HashMap with the binary mapping 
/// of the available destination instructions. It follows the HACK-spec.
/// The function's return value works like a contstant.
///
/// # Usage
/// ```
/// use hack_assembler::tables::get_dest_table;
/// use std::collections::HashMap;
///
/// println!("{:?}", get_dest_table());
/// ```
pub fn get_dest_table<'a>() -> HashMap<&'a str, &'a str> {
    let mut dest_options = HashMap::new();
    dest_options.insert("null", "000");
    dest_options.insert("M", "001");
    dest_options.insert("D", "010");
    dest_options.insert("MD", "011");
    dest_options.insert("A", "100");
    dest_options.insert("AM", "101");
    dest_options.insert("AD", "110");
    dest_options.insert("AMD", "111");
    dest_options
}

/// Works the same as `get_dest_table`, only for jump.
/// The function returns a HashMap with the binary mapping 
/// of the available jump instruction. It follows the HACK-spec.
/// The function's return value works like a contstant.
///
/// # Usage
/// ```
/// use hack_assembler::tables::get_jmp_table;
/// use std::collections::HashMap;
///
/// println!("{:?}", get_jmp_table());
/// ```
pub fn get_jmp_table<'a>() -> HashMap<&'a str, &'a str> {
    let mut jmp_options = HashMap::new();
    jmp_options.insert("null", "000");
    jmp_options.insert("JGT", "001");
    jmp_options.insert("JEQ", "010");
    jmp_options.insert("JGE", "011");
    jmp_options.insert("JLT", "100");
    jmp_options.insert("JNE", "101");
    jmp_options.insert("JLE", "110");
    jmp_options.insert("JMP", "111");
    jmp_options
}

/// The function returns a HashMap with the binary mapping 
/// of the available computation-instruction (where a == 0).
/// It follows the HACK-spec.
/// The function's return value works like a contstant.
///
/// # Usage
/// ```
/// use hack_assembler::tables::get_comp_table_not_a;
/// use std::collections::HashMap;
///
/// println!("{:?}", get_comp_table_not_a());
/// ```
pub fn get_comp_table_not_a<'a>() -> HashMap<&'a str, &'a str> {
    let mut comp_options = HashMap::new();
    comp_options.insert("0", "101010");
    comp_options.insert("1", "111111");
    comp_options.insert("-1", "111010");
    comp_options.insert("D", "001100");
    comp_options.insert("A", "110000");
    comp_options.insert("!D", "001101");
    comp_options.insert("!A", "110001");
    comp_options.insert("-D", "001111");
    comp_options.insert("-A", "110011");
    comp_options.insert("D+1", "011111");
    comp_options.insert("A+1", "110111");
    comp_options.insert("D-1", "001110");
    comp_options.insert("A-1", "110010");
    comp_options.insert("D+A", "000010");
    comp_options.insert("D-A", "010011");
    comp_options.insert("A-D", "000111");
    comp_options.insert("D&A", "000000");
    comp_options.insert("D|A", "010101");
    comp_options
}

/// The function returns a HashMap with the binary mapping 
/// of the available computation-instruction (where a == 1).
/// It follows the HACK-spec.
/// The function's return value works like a contstant.
///
/// # Usage
/// ```
/// use hack_assembler::tables::get_comp_table_a;
/// use std::collections::HashMap;
///
/// println!("{:?}", get_comp_table_a());
/// ```
pub fn get_comp_table_a<'a>() -> HashMap<&'a str, &'a str> {
    let mut comp_options = HashMap::new();
    comp_options.insert("M", "110000");
    comp_options.insert("!M", "110001");
    comp_options.insert("-M", "110001");
    comp_options.insert("M+1", "110111");
    comp_options.insert("M-1", "110010");
    comp_options.insert("D+M", "000010");
    comp_options.insert("D-M", "010011");
    comp_options.insert("M-D", "000111");
    comp_options.insert("D&M", "000000");
    comp_options.insert("D|M", "010101");
    comp_options
}