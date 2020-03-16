use crate::tokenizer::token::{ Token, TokenType };
use crate::compiler::tables::{ Var };
use crate::compiler::{ Subroutine, ReturnType };

pub fn write_push(kind: &str, typ: &str, idx_or_value: u32) -> String {
    match kind {
        "constant" => format!("push constant {}", idx_or_value),
        "arg" => format!("push argument {}", idx_or_value),
        "local" => format!("push local {}", idx_or_value),
        _ => format!("Write_push received type: '{}', value '{}' which is not implemented.", kind, idx_or_value),
    }
}

pub fn write_pop(kind: &str, typ: &str, idx: u32) -> String {
    match kind {
        "arg" => format!("pop argument {}", idx),
        "local" => format!("pop local {}", idx),
        _ => format!("Write_pop received type: '{}', value '{}' which is not implemented.", kind, idx),
    }
}

pub fn write_op(operator: &Token) -> String {
    match operator.value.as_ref() {
        "+" => String::from("add"),
        "-" => String::from("sub"),
        "*" => String::from("call Math.multiply 2"),
        "/" => String::from("call Math.devide 2"),
        "<" => String::from("lt"),
        ">" => String::from("gt"),
        "=" => String::from("eq"),
        "&" => String::from("and"),
        "|" => String::from("or"),
        op => format!("Operator '{}' is not implemented.", op),
    }
}

pub fn write_return(subroutine: &Subroutine) -> String {
    match &subroutine.return_type {
        ReturnType::Void => String::from("push constant 0"),
        _ => String::from(""),
        // rt => format!("RETURN_TYPE of '{:?}' IS NOT IMPLEMENTED YET", rt),
    }
}

pub fn write_unary_op(token: &Token) -> String {
    match token.value.as_ref() {
        "-" => String::from("neg"),
        "~" => String::from("not"),
        op => panic!("Expected unary-operator but got '{}'", op),
    }
}

// TESTS

// Push
#[test]
fn write_push_integer_constant() {
    let Var {kind, typ, idx} = Var::new("constant", "_", 1);
    let dummy_result = "push constant 1".to_string();
    assert_eq!(dummy_result, write_push(&kind, &typ, idx));
}

#[test]
fn write_push_argument() {
    let Var {kind, typ, idx} = Var::new("arg", "int", 0);
    // TODO: Result is more complex
    let dummy_result = "push arg 0".to_string();
    assert_eq!(dummy_result, write_push(&kind, &typ, idx));
}

// Pop
#[test]
fn write_pop_assignment() {
    let Var {kind, typ, idx} = Var::new("local", "int", 0);
    // TODO: Result is more complex
    let dummy_result = "pop local 0".to_string();
    assert_eq!(dummy_result, write_pop(&kind, &typ, idx));
}