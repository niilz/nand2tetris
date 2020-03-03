use super::token:: { Token, TokenType };

pub fn write_push(term: &Token) -> String {
    match term.token_type {
        TokenType::IntegerConstant => format!("push constant {}", term.value),
        _ => format!("Write_push received type: '{}', value '{}' which is not implemented.", term.token_type, term.value),
    }
}

pub fn write_op(operator: &Token) -> String {
    match operator.value.as_ref() {
        "+" => String::from("add"),
        "-" => String::from("sub"),
        "*" => String::from("call Math.multiply 2"),
        "/" => String::from("call Math.devide 2"),
        op => format!("Operator '{}' is not implemented.", op),
    }
}

pub fn write_return(return_type: &str) -> String {
    String::from("pop temp 0")
}