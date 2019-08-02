// Docs can be opened with "cargo doc --open"
pub mod parser;
pub mod tables;

pub mod encoder {
    //! Takes care of the correct binary-translation
    //! for every CommandType.
    
    use crate::parser::CommandType;
    use crate::parser::CommandType::{ ACommand, CCommand, LCommand };
    use crate::tables::*;

    /// Uses the `parser` module to translate the different `CommandType`s.
    /// It also uses the `tables` module to get the mapping for certain instructions.
    /// In case of an `LCommand` variant, a `#` is put in front of the `&str` value.
    /// This is usefull to later identify labels and variables.
    /// 
    /// # Examples
    /// ```
    /// use hack_assembler::parser::CommandType::{ACommand, CCommand, LCommand};
    /// use hack_assembler::encoder::translate_instruction;
    ///
    /// assert_eq!(translate_instruction(ACommand("11")), "0000000000001011");
    /// assert_eq!(translate_instruction(LCommand("loop")), "#loop");
    /// assert_eq!(translate_instruction(CCommand{dest: Some("M"), comp: Some("!A"), jmp: None}), "1110110001001000");
    ///
    pub fn translate_instruction<'a>(inst: CommandType) -> String {
        match inst {
            ACommand(value) => get_bin_value(&value),
            LCommand(value) => "#".to_owned() + value,
            CCommand{dest, comp, jmp} => translate_c_command(dest, comp, jmp),
        }
    }

    /// Receives an instruction which is either a value or still variable.
    /// If it is a value: the function parses the numeric `&str` into a number
    /// and returns it as a 16bit binary `String`.
    /// If the parsing does not work (the value is not numeric and must still be
    /// representing a variable), the function attaches an `*` infront of
    /// of the `&str` value and returns it as `String`. The `*` helps later to
    /// distinguish between label (`#some-label`) and a variable (`*some-variable`).
    /// 
    /// # Examples
    ///
    /// ```
    /// use hack_assembler::encoder::get_bin_value;
    ///
    /// assert_eq!(get_bin_value("7"), "0000000000000111");
    /// assert_eq!(get_bin_value("2013"), "0000011111011101");
    /// assert_eq!(get_bin_value("R11"), "*R11");
    /// assert_eq!(get_bin_value("counter"), "*counter");
    /// ```
    pub fn get_bin_value(a_inst: &str) -> String {
        match a_inst.parse::<i32>() {
            Ok(value) => format!("{:0>16b}", value),
            Err(_) => format!("*{}", a_inst),
        }
    }

    // A helper to build a final binary representation of a C-instruction.
    fn translate_c_command<'a>(dest: Option<&str>, comp: Option<&str>, jmp: Option<&str>) -> String {

        let mut c_inst_bin = String::from("111");

        match comp {
            Some(comp) => c_inst_bin.push_str(&get_comp(comp)),
            None => c_inst_bin.push_str("0000000"),
        };
        match dest {
            Some(dest) => c_inst_bin.push_str(get_dest(dest)),
            None => c_inst_bin.push_str("000"),
        };
        match jmp {
            Some(jmp) => c_inst_bin.push_str(get_jmp(jmp)),
            None => c_inst_bin.push_str("000"),
        }
        c_inst_bin
    }

    // Helper-functions that access the tables module and
    // deliver the correct binary values for destination,
    // computation and jump fields of a C-instruction.
    fn get_dest<'a>(dest: &str) -> &'a str {
        let dest_table = get_dest_table();
        dest_table[dest]
    }
    fn get_comp<'a>(comp: &str) -> String {
        let comp_table_a = get_comp_table_not_a();
        let comp_table_b = get_comp_table_a();
        if comp_table_a.contains_key(comp) {
            "0".to_owned() + comp_table_a[comp]
        } else {
            "1".to_owned() + comp_table_b[comp]
        }
    }
    fn get_jmp<'a>(jmp: &str) -> &'a str {
        let jmp_table = get_jmp_table();
        jmp_table[jmp]
    }

    #[cfg(test)]
    mod test {
        use super::*;

        #[test]
        fn turns_a_inst_into_bin_value_1() {
            assert_eq!(get_bin_value("100"), "0000000001100100");
        }

        // test instruction translation
        #[test]
        fn translates_a_command() {
            let test_inst = ACommand("4");
            assert_eq!(translate_instruction(test_inst), "0000000000000100");
        }

        // Test instruction-part getters
        #[test]
        fn gets_correct_comp_a() {
            assert_eq!(get_comp("D-A"), "0010011");
        }
        #[test]
        fn gets_correct_comp_b() {
            assert_eq!(get_comp("M+1"), "1110111");
        }
        #[test]
        fn gets_correct_dest() {
            assert_eq!(get_dest("A"), "100");
        }
        #[test]
        fn gets_correct_dest_none() {
            assert_eq!(get_dest("null"), "000");
        }
        #[test]
        fn gets_correct_jmp() {
            assert_eq!(get_jmp("JLT"), "100");
        }
        #[test]
        fn gets_correct_jmp_none() {
            assert_eq!(get_jmp("null"), "000");
        }

        // test c-instruction translation
        #[test]
        fn translate_c_inst_all_none() {
            assert_eq!(translate_c_command(None, None, None), "1110000000000000")
        }
        #[test]
        fn translate_c_inst_dest() {
            assert_eq!(translate_c_command(Some("AM"), None, None), "1110000000101000")
        }

        // test GENERAL-Translation works
        #[test]
        fn translates_c_instruction_a() {
            assert_eq!(translate_instruction(CCommand{dest: Some("AM"), comp: Some("D|A"), jmp: Some("JEQ")}), "1110010101101010");
        }
        #[test]
        fn translates_c_instruction_b() {
            assert_eq!(translate_instruction(CCommand{dest: Some("AMD"), comp: Some("D&M"), jmp: Some("null")}), "1111000000111000");
        }
        #[test]
        fn translates_a_instruction() {
            assert_eq!(translate_instruction(ACommand("9")), "0000000000001001");
        }
    }
}