use crate::tokenizer::token::{ Token };
use crate::compiler::{ Subroutine, ReturnType };

pub fn write_push(kind: &str, idx_or_value: u32) -> String {
    match kind {
        "constant" => format!("push constant {}", idx_or_value),
        "argument" => format!("push argument {}", idx_or_value),
        "local" => format!("push local {}", idx_or_value),
        "field" => format!("push this {}", idx_or_value),
        "static" => format!("push static {}", idx_or_value),
        _ => format!("Write_push received type: '{}', value '{}' which is not implemented.", kind, idx_or_value),
    }
}

pub fn write_pop(kind: &str, idx: u32) -> String {
    match kind {
        "argument" => format!("pop argument {}", idx),
        "local" => format!("pop local {}", idx),
        "field" => format!("pop this {}", idx),
        "static" => format!("pop static {}", idx),
        _ => format!("Write_pop received type: '{}', value '{}' which is not implemented.", kind, idx),
    }
}

pub fn write_op(operator: &Token) -> String {
    match operator.value.as_ref() {
        "+" => String::from("add"),
        "-" => String::from("sub"),
        "*" => String::from("call Math.multiply 2"),
        "/" => String::from("call Math.divide 2"),
        "<" => String::from("lt"),
        ">" => String::from("gt"),
        "=" => String::from("eq"),
        "&" => String::from("and"),
        "|" => String::from("or"),
        op => format!("Operator '{}' is not implemented.", op),
    }
}

pub fn write_unary_op(token: &Token) -> String {
    match token.value.as_ref() {
        "-" => String::from("neg"),
        "~" => String::from("not"),
        op => panic!("Expected unary-operator but got '{}'", op),
    }
}

pub fn write_return(subroutine: &Subroutine) -> String {
    match &subroutine.return_type {
        ReturnType::Void => String::from("push constant 0"),
        _ => String::from("// non_void_no_dummy_0"),
        // rt => format!("RETURN_TYPE of '{:?}' IS NOT IMPLEMENTED YET", rt),
    }
}

pub fn write_string(string: &str) -> Vec<String> {
    let mut commands = Vec::new();
    let len = string.len();
    commands.push(format!("push constant {}", len));
    commands.push("call String.new 1".to_string());
    string
        .chars()
        .for_each(|c| {
            commands.push(format!("push constant {}", c as u8));
            commands.push("call String.appendChar 2".to_string());
        });
    commands
}

pub fn write_array_assignment() -> Vec<String> {
    let mut commands = Vec::new();
    // Save expression on right side of let-assignment to temp
    commands.push("pop temp 0".to_string());
    // Put left side expression into pointer 1 (the that segment)
    commands.push("pop pointer 1".to_string());
    // Push expression in temp back onto stack
    commands.push("push temp 0".to_string());
    // Put right side expression into var on left side
    commands.push("pop that 0".to_string());

    commands
}