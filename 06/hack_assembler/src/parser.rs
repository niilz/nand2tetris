//! Takes care of the line-cleaning functionality,
//! `CommandType` categorization and
//! seperation of instruction-fields.

extern crate regex;

use regex::{ Regex, Captures };

/// The `CommandType`has three different variants.
/// One for each possible HACK instruction (A-Instruction, C-Instruction, L-Instruction)
///
#[derive(Debug)]
#[derive(PartialEq)]
pub enum CommandType<'a> {
    ACommand(&'a str),
    CCommand {dest: Option<&'a str>, comp: Option<&'a str>, jmp: Option<&'a str>},
    LCommand(&'a str),
}

/// Cleanes line, so that only instruction gets kept.
///
/// # Examples
///
/// ```
/// extern crate hack_assembler;
/// use hack_assembler::parser::clean_line;
///
/// assert_eq!(clean_line("A=D+1 // Comment explaining the line."), "A=D+1");
/// ```
pub fn clean_line(line: &str) -> &str {
    let trimmed = line.trim();
    let no_comments: Vec<&str> = trimmed.split("//").collect();
    no_comments[0].trim()
}

/// Return a `CommandType-Variant` (ACommand, CCommand or LCommand),
/// based on the instruction it receivess. Ideally the instruction has no
/// white space or comment. So it is useful to first use `clean_line` on the
/// instruction befor it get's passed to get_command_fields.
///
/// # Examples
/// ```
/// use hack_assembler::parser::{CommandType, get_command_fields};
///
/// let instruction = "@100";
/// let command = get_command_fields(&instruction);
/// assert_eq!(command, CommandType::ACommand("100"));
///
pub fn get_command_fields(instruction: &str) -> CommandType {
    let mut inst_chars = instruction.chars();
    let len = instruction.len();

    match inst_chars.next().unwrap() {
        '@' => CommandType::ACommand(&instruction[1..]),
        '(' => CommandType::LCommand(&instruction[1..len-1]),
        _ => get_ccom_fields(instruction),
    }
}

/// Expects an Instruction that is no A or L Type.
/// Therefore it must be a C-Command.
/// The function populates the CCommand Variant by
/// regexing through the cleaned line.
///
/// #Examples
///
///```
/// use hack_assembler::parser::{get_ccom_fields, CommandType};
///
/// assert_eq!(get_ccom_fields("A=M+1; JGT"), CommandType::CCommand{dest: Some("A"), comp: Some("M+1"), jmp: Some("JGT")});
/// assert_eq!(get_ccom_fields("D-1"), CommandType::CCommand{dest: None, comp: Some("D-1"), jmp: None});
/// 
pub fn get_ccom_fields(instruction: &str) -> CommandType {
    
    let c_com_regex = Regex::new(r"((?P<dest>^[DMA]{1,3}|null)=)?(?P<comp>[DMA01!&|+-]{1,3})(;\s*?(?P<jmp>[A-Z]{1,3}|null))?").unwrap();
    let fields = c_com_regex.captures(instruction).unwrap();

    CommandType::CCommand {
        dest: unpack_match("dest", &fields),
        comp: unpack_match("comp", &fields),
        jmp: unpack_match("jmp", &fields),
    }
}

/// Helper (used by `parser::get_ccom_field` to unpack the C-Instruction-field
/// using regex-name-matches.
///
fn unpack_match<'a>(name: &str, fields: &Captures<'a>) -> Option<&'a str> {
    match fields.name(name) {
        Some(field) => Some(field.as_str()),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn keeps_pure_a_instruction() {
        assert_eq!(clean_line("@100"), "@100");
    }
    #[test]
    fn ignores_empty_line() {
        assert_eq!(clean_line("\n"), "");
    }
    #[test]
    fn understands_complex_a_instruction() {
        assert_eq!(clean_line("@ponggame.newinstance"), "@ponggame.newinstance");
    }
    #[test]
    fn understands_complex_label() {
        assert_eq!(clean_line("(bat.move$if_end0)"), "(bat.move$if_end0)");
    }
    #[test]
    fn ignores_comment() {
        assert_eq!(clean_line("// This is a comment"), "");
    }
    #[test]
    fn keeps_pure_c_instruction_without_jmp() {
        assert_eq!(clean_line("M=A+1"), "M=A+1");
    }
    #[test]
    fn deletes_comment_behind_instruction() {
        assert_eq!(clean_line("A=D-M  // comment in line"), "A=D-M");
    }
    #[test]
    fn keeps_c_instruction_all_fields() {
        assert_eq!(clean_line("D=A+1; JEQ"), "D=A+1; JEQ");
    }
    #[test]
    fn keeps_label() {
        assert_eq!(clean_line("(loop)"), "(loop)");
    }
    #[test]
    fn keeps_variable() {
        assert_eq!(clean_line("@i"), "@i");
    }

    // TESTS for get A-Command
    #[test]
    fn finds_value() {
        assert_eq!(get_command_fields("@5"), CommandType::ACommand("5"));
    }
    #[test]
    fn finds_register() {
        assert_eq!(get_command_fields("@R0"), CommandType::ACommand("R0"));
    }
    #[test]
    fn gets_complex_a_command_field() {
        assert_eq!(get_command_fields("@ponggame.newinstance"), CommandType::ACommand("ponggame.newinstance"));;
    }

    // TESTS for get L-Command
    #[test]
    fn gets_complex_l_command_field() {
        assert_eq!(get_command_fields("(bat.move$if_end0)"), CommandType::LCommand("bat.move$if_end0"));;
    }

    // TESTS for get C-Command
    #[test]
    fn get_dest_comp_and_jmp() {
        assert_eq!(
            get_ccom_fields("D=M; JNE"),
            CommandType::CCommand{dest: Some("D"), comp: Some("M"), jmp: Some("JNE")});
    }
    #[test]
    fn get_dest_and_comp() {
        assert_eq!(
            get_ccom_fields("A=A-D"),
            CommandType::CCommand{dest: Some("A"), comp: Some("A-D"), jmp: None});
    }
    #[test]
    fn get_comp() {
        assert_eq!(
            get_ccom_fields("D+1"),
            CommandType::CCommand{dest: None, comp: Some("D+1"), jmp: None});
    }
    #[test]
    fn accounts_for_null_as_dest() {
        assert_eq!(
            get_ccom_fields("null=D+M; JNE"),
            CommandType::CCommand{dest: Some("null"), comp: Some("D+M"), jmp: Some("JNE")});
    }
    #[test]
    fn accounts_for_null_as_jmp() {
        assert_eq!(
            get_ccom_fields("D+M; null"),
            CommandType::CCommand{dest: None, comp: Some("D+M"), jmp: Some("null")});
    }
    #[test]
    fn get_multi_dest_comp_jmp() {
        assert_eq!(
            get_ccom_fields("DM=A+1; JMP"),
            CommandType::CCommand{dest: Some("DM"), comp: Some("A+1"), jmp: Some("JMP")});
    }
    #[test]
    fn get_multi_dest_no_jmp() {
        assert_eq!(
            get_ccom_fields("DM=A+1"),
            CommandType::CCommand{dest: Some("DM"), comp: Some("A+1"), jmp: None});
    }
    #[test]
    fn get_negotiation_of_m_no_jmp() {
        assert_eq!(
            get_ccom_fields("M=!M"),
            CommandType::CCommand{dest: Some("M"), comp: Some("!M"), jmp: None});
    }
}