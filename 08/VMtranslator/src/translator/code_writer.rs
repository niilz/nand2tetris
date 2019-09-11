use std::path::Path;

// ASM-code-generator-functions
fn sp_down() -> String {
    "@SP AM=M-1".to_string()
}
fn sp_up() -> String {
    "@SP M=M+1".to_string()
}

// Translates parsed Arithmetic commands (Com::Arith) into HACK-ASM
pub fn write_arithmetic(method: &str, line_nr: usize) -> String {

    let comment = format!("\n// {}\n", method);

    let arith_method = match method {
        "add" | "sub" | "or" | "and" => add_sub_or_and(&method),
        "eq" | "gt" | "lt" => eq_gt_lt(&method, line_nr),
        "neg" => "D=0 M=D-M".to_string(),
        "not" => "M=!M".to_string(),
        _ => panic!("The arithmetic command contained the unknown method: {}", method),
    };

    let asm_string = format!("{} {} {}", sp_down(), arith_method, sp_up());
    comment + &asm_new_line_concat(&asm_string)
}


// Constructs the middle-part of an "add, sub, or, and" command.
fn add_sub_or_and(method: &str) -> String {

    let command = match method {
        "add" => "M=M+D",
        "sub" => "M=M-D",
        "or" => "M=D|M",
        "and" => "M=D&M",
        _ => panic!("The unknown method '{}' has been passed into 'add_sub_or_and'."),
    };
    format!("D=M {} {}", sp_down(), command)
}

// Gets an "eg, gt or lt" command and a line_nr which is used to create
// uniquie label-names for the jmp-instructions.
fn eq_gt_lt(method: &str, label_nr: usize) -> String {
    let front = format!("D=M @SP AM=M-1 D=M-D @True{}", label_nr);
    let tail = format!("@SP A=M M=0 @Next{} 0;JMP (True{}) @SP A=M D=0 M=D-1 (Next{})", label_nr, label_nr, label_nr);
    let middle = match method {
        "eq" => "D;JEQ",
        "gt" => "D;JGT",
        "lt" => "D;JLT",
        _ => panic!("The unknown method '{}' has been passed into 'eq_gt_lt'."),
    };
    format!("{} {} {}", front, middle, tail)
}

// Translates parsed push-commands (Com::Push) into HACK-ASM
pub fn write_push(segment: &str, position: u32, file: &str) -> String {
    let comment = format!("\n// push {} {}\n", segment, position);
    
    if segment == "static" {
        let asm_segment = format!("@{}.{} D=M @SP A=M M=D {}", file, position, sp_up());
        return comment + &asm_new_line_concat(&asm_segment)
    }

    let segment_asm = match segment {
        "constant" => "D=A",
        "local" => "D=A @LCL A=D+M D=M",
        "argument" => "D=A @ARG A=D+M D=M",
        "this" => "D=A @THIS A=D+M D=M",
        "that" => "D=A @THAT A=D+M D=M",
        "temp" => "D=A @5 A=D+A D=M",
        "pointer" => match position {
            0 => "@THIS D=M",
            1 => "@THAT D=M",
            _ => panic!("A pointer value of '{}' cannot be pushed from THIS or THAT.", position),
        },
        _ => {
            println!("push not impemented: {}", segment);
            "push_not_yet_implemented! See_print-statement."
        }
    };

    let asm_string = format!("@{} {} @SP A=M M=D {}", position, segment_asm, sp_up());
    comment + &asm_new_line_concat(&asm_string)
}

// Translates parsed pop-commands (Com::Pop) into HACK-ASM
pub fn write_pop(segment: &str, position: u32, dest_id: usize, file: &str) -> String {
    let comment = format!("\n// pop {} {}\n", segment, position);
    
    if segment == "pointer" {
        let segment_asm = match position {
            0 => format!("{} @SP A=M D=M @THIS M=D", sp_down()),
            1 => format!("{} @SP A=M D=M @THAT M=D", sp_down()),
            _ => panic!("A position of: '{}' cannot be popped into THIS or THAT"),
        };
        return comment + &asm_new_line_concat(&segment_asm);
    } else if segment == "static" {
        let segment_asm = format!("{} @SP A=M D=M @{}.{} M=D", sp_down(), file, position);
        return comment + &asm_new_line_concat(&segment_asm);
    }

    let segment_asm = match segment {
        "local" => "LCL D=D+M",
        "argument" => "ARG D=D+M",
        "this" => "THIS D=D+M",
        "that" => "THAT D=D+M",
        "temp" => "5 D=D+A",
        _ => {
            println!("push not impemented: {}", segment);
            "push not yet implemented! See print-statement."
        }
    };

    let asm_string = format!("{} @{} D=A @{} @dest{} M=D @SP A=M D=M @dest{} A=M M=D", sp_down(), position, segment_asm, dest_id, dest_id);
    comment + &asm_new_line_concat(&asm_string)
}

pub fn write_label(label: &str) -> String {
    format!("\n({})", label)
}
pub fn write_branch(condition: &str, label: &str) -> String {
    let comment = format!("\n// JMP to LABEL: {}\n", label);
    let condition_asm = match condition {
        "goto" => format!("{} @{} 0;JMP", sp_down(), label),
        "if-goto" => format!("{} D=M @{} D;JNE", sp_down(), label),
        _ => panic!("Unknown branching command: '{}' has been parsed to 'write_branch'.", condition),
    };
    comment + &asm_new_line_concat(&condition_asm)
}

// Helper to split a string (on whitespace) and concat it again with \n .
fn asm_new_line_concat(asm_string: &str) -> String {
    asm_string.split(" ").collect::<Vec<&str>>().join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;

    // test translation of arithmetic commands to ASM
    #[test]
    fn add_com() {
        assert_eq!(write_arithmetic("add", 1), "\n// add\n@SP\nAM=M-1\nD=M\n@SP\nAM=M-1\nM=M+D\n@SP\nM=M+1");
    }
    #[test]
    fn sub_com() {
        assert_eq!(write_arithmetic("sub", 1), "\n// sub\n@SP\nAM=M-1\nD=M\n@SP\nAM=M-1\nM=M-D\n@SP\nM=M+1");
    }
    #[test]
    fn neg_com() {
        assert_eq!(write_arithmetic("neg", 1), "\n// neg\n@SP\nAM=M-1\nD=0\nM=D-M\n@SP\nM=M+1");
    }
    #[test]
    fn eq_com() {
        assert_eq!(write_arithmetic("eq", 1), "\n// eq\n@SP\nAM=M-1\nD=M\n@SP\nAM=M-1\nD=M-D\n@True1\nD;JEQ\n@SP\nA=M\nM=0\n@Next1\n0;JMP\n(True1)\n@SP\nA=M\nD=0\nM=D-1\n(Next1)\n@SP\nM=M+1");
    }
    #[test]
    fn gt_com() {
        assert_eq!(write_arithmetic("gt", 2), "\n// gt\n@SP\nAM=M-1\nD=M\n@SP\nAM=M-1\nD=M-D\n@True2\nD;JGT\n@SP\nA=M\nM=0\n@Next2\n0;JMP\n(True2)\n@SP\nA=M\nD=0\nM=D-1\n(Next2)\n@SP\nM=M+1");
    }
    #[test]
    fn lt_com() {
        assert_eq!(write_arithmetic("lt", 3), "\n// lt\n@SP\nAM=M-1\nD=M\n@SP\nAM=M-1\nD=M-D\n@True3\nD;JLT\n@SP\nA=M\nM=0\n@Next3\n0;JMP\n(True3)\n@SP\nA=M\nD=0\nM=D-1\n(Next3)\n@SP\nM=M+1");
    }
    #[test]
    fn or_com() {
        assert_eq!(write_arithmetic("or", 0), "\n// or\n@SP\nAM=M-1\nD=M\n@SP\nAM=M-1\nM=D|M\n@SP\nM=M+1");
    }
    #[test]
    fn and_com() {
        assert_eq!(write_arithmetic("and", 0), "\n// and\n@SP\nAM=M-1\nD=M\n@SP\nAM=M-1\nM=D&M\n@SP\nM=M+1");
    }
    #[test]
    fn not_com() {
        assert_eq!(write_arithmetic("not", 0), "\n// not\n@SP\nAM=M-1\nM=!M\n@SP\nM=M+1");
    }
    #[test]
    #[should_panic]
    fn panics_on_unknown_method() {
        write_arithmetic("blbla", 0);
    }

    // Tests push-commands
    #[test]
    fn push_static_works() {
        let comment = "\n// push constant 99\n".to_string();
        let push_static_99_string = asm_new_line_concat("@99 D=A @SP A=M M=D @SP M=M+1");
        assert_eq!(write_push("constant", 99, ""), comment + &push_static_99_string);
    }
    #[test]
    fn push_local_works() {
        assert_eq!(write_push("local", 0, ""), "\n// push local 0\n@0\nD=A\n@LCL\nA=D+M\nD=M\n@SP\nA=M\nM=D\n@SP\nM=M+1");
    }
    #[test]
    fn push_temp_works() {
        assert_eq!(write_push("temp", 6, ""), "\n// push temp 6\n@6\nD=A\n@5\nA=D+A\nD=M\n@SP\nA=M\nM=D\n@SP\nM=M+1");
    }
    #[test]
    fn push_arg_works() {
        assert_eq!(write_push("argument", 1, ""), "\n// push argument 1\n@1\nD=A\n@ARG\nA=D+M\nD=M\n@SP\nA=M\nM=D\n@SP\nM=M+1");
    }
    #[test]
    fn push_this_works() {
        assert_eq!(write_push("this", 6, ""), "\n// push this 6\n@6\nD=A\n@THIS\nA=D+M\nD=M\n@SP\nA=M\nM=D\n@SP\nM=M+1");
    }
    #[test]
    fn push_that_works() {
        assert_eq!(write_push("that", 5, ""), "\n// push that 5\n@5\nD=A\n@THAT\nA=D+M\nD=M\n@SP\nA=M\nM=D\n@SP\nM=M+1");
    }
    #[test]
    fn push_pointer_0_works() {
        assert_eq!(write_push("pointer", 0, ""), "\n// push pointer 0\n@0\n@THIS\nD=M\n@SP\nA=M\nM=D\n@SP\nM=M+1");
    }
    #[test]
    fn push_pointer_1_works() {
        assert_eq!(write_push("pointer", 1, ""), "\n// push pointer 1\n@1\n@THAT\nD=M\n@SP\nA=M\nM=D\n@SP\nM=M+1");
    }
    #[test]
    fn push_static_88_works() {
        assert_eq!(write_push("static", 88, "halloele"), "\n// push static 88\n@halloele.88\nD=M\n@SP\nA=M\nM=D\n@SP\nM=M+1");
    }

    // Tests pop-commands
    #[test]
    fn pop_to_local() {
        assert_eq!(write_pop("local", 0, 1, ""), "\n// pop local 0\n@SP\nAM=M-1\n@0\nD=A\n@LCL\nD=D+M\n@dest1\nM=D\n@SP\nA=M\nD=M\n@dest1\nA=M\nM=D");
    }
    #[test]
    fn pop_to_arg() {
        assert_eq!(write_pop("argument", 1, 5, ""), "\n// pop argument 1\n@SP\nAM=M-1\n@1\nD=A\n@ARG\nD=D+M\n@dest5\nM=D\n@SP\nA=M\nD=M\n@dest5\nA=M\nM=D");
    }
    #[test]
    fn pop_to_temp() {
        assert_eq!(write_pop("temp", 6, 13, ""), "\n// pop temp 6\n@SP\nAM=M-1\n@6\nD=A\n@5\nD=D+A\n@dest13\nM=D\n@SP\nA=M\nD=M\n@dest13\nA=M\nM=D");
    }
    #[test]
    fn pop_to_this() {
        assert_eq!(write_pop("this", 6, 7, ""), "\n// pop this 6\n@SP\nAM=M-1\n@6\nD=A\n@THIS\nD=D+M\n@dest7\nM=D\n@SP\nA=M\nD=M\n@dest7\nA=M\nM=D");
    }
    #[test]
    fn pop_to_that() {
        assert_eq!(write_pop("that", 5, 10, ""), "\n// pop that 5\n@SP\nAM=M-1\n@5\nD=A\n@THAT\nD=D+M\n@dest10\nM=D\n@SP\nA=M\nD=M\n@dest10\nA=M\nM=D");
    }
    #[test]
    fn pop_pointer_0() {
        assert_eq!(write_pop("pointer", 0, 0, ""), "\n// pop pointer 0\n@SP\nAM=M-1\n@SP\nA=M\nD=M\n@THIS\nM=D");
    }
    #[test]
    fn pop_pointer_1() {
        assert_eq!(write_pop("pointer", 1, 0, ""), "\n// pop pointer 1\n@SP\nAM=M-1\n@SP\nA=M\nD=M\n@THAT\nM=D");
    }
    #[test]
    fn pop_static_5() {
        assert_eq!(write_pop("static", 5, 0, "bla"), "\n// pop static 5\n@SP\nAM=M-1\n@SP\nA=M\nD=M\n@bla.5\nM=D");
    }

    // Test Branch-commands
    #[test]
    fn if_goto_label_works() {
        assert_eq!(write_branch("if-goto", "LABEL_BAMBI"), "\n// JMP to LABEL: LABEL_BAMBI\n@SP\nAM=M-1\nD=M\n@LABEL_BAMBI\nD;JNE");
    }
    #[test]
    fn goto_label_works() {
        assert_eq!(write_branch("goto", "LABEL_BAMBI"), "\n// JMP to LABEL: LABEL_BAMBI\n@SP\nAM=M-1\n@LABEL_BAMBI\nNULL;JMP");
    }
    // Test Label-command
    #[test]
    fn label_my_label_works() {
        assert_eq!(write_label("A_GREAT_LABEL"), "\n(A_GREAT_LABEL)");
    }

    // Helper-functions
    #[test]
    fn concat_asm_string_with_new_line() {
        assert_eq!(asm_new_line_concat("@sp test works just fine"), "@sp\ntest\nworks\njust\nfine");
    }
}