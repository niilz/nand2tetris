#[derive(Debug)]
#[derive(PartialEq)]
// Enum with com-Variantss
pub enum Com {
    Empty,
    Push(String, u32),
    Pop(String, u32),
    Arith(String),
    Label(String),
    Branch(String, String),
    Function(String, u32),
    Return,
}

// Cleanes a given line (e.g. from comments)
fn clean_line(line: &str) -> String {
    let trimmed = line.trim();
    if trimmed.is_empty() || &trimmed[0..2] == "//" {
        return "".to_string();
    }

    let line: Vec<&str> = trimmed.split("//").collect(); 
    line[0].trim().to_string()
}

// parses a given line -> returns a com::Variant
pub fn parse_line(line: &str) -> Com {
    let cleaned_line = clean_line(line);
    let com = cleaned_line.trim();

    if com.is_empty() {
        return Com::Empty;
    }

    let com_fields: Vec<&str> = com.split(" ").collect();

    match com_fields[..] {
        // Only 1 arg means Arithmetic or Return command
        [arg0] => {
            match arg0 {
                "return" => Com::Return,
                arith => Com::Arith(arith.to_string()),
            }
        },
        // Two args means Label or Branch
        [arg0, arg1] => {
            match arg0 {
                "label" => Com::Label(arg1.to_string()),
                goto_ifgoto => Com::Branch(goto_ifgoto.to_string(), arg1.to_string()),
            }
        },
        // Three args means Push, Pop or Function command
        [arg0, arg1, arg2] => {
            let arg2 = match arg2.parse::<u32>() {
                Ok(value) => value,
                Err(message) => panic!("Arg2 '{}' (3rd arg) could not be parsed. Error: {}", arg2, message),
            };
            match arg0 {
                "function" => Com::Function(arg1.to_string(), arg2),
                "push" => Com::Push(arg1.to_string(), arg2),
                "pop" => Com::Pop(arg1.to_string(), arg2),
                _ => panic!("Expected command to be push, pop or function, but the command was neither."),
            }
        },
        _ => panic!("The given line was '{}'. It could not be parsed.", line),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Tests clean_line()
    #[test]
    fn removes_comment_line() {
        assert_eq!(clean_line("// some nasty comment"), "".to_string());
    }
    #[test]
    fn removes_comment_behind_com() {
        assert_eq!(clean_line("push 2   // some nasty comment behind"), "push 2".to_string());
    }
    #[test]
    fn removes_comment_behind_com2() {
        assert_eq!(clean_line("pop local 0         // initializes sum = 0"), "pop local 0".to_string());
    }
    #[test]
    fn removes_empty_line() {
        assert_eq!(clean_line("\n"), "".to_string());
    }

    // Test parse_line()
    #[test]
    fn returns_push_com() {
        assert_eq!(parse_line("push local 2"), Com::Push("local".to_string(), 2));
    }
    #[test]
    fn returns_pop_com() {
        assert_eq!(parse_line("pop static 3"), Com::Pop("static".to_string(), 3));
    }
    #[test]
    fn returns_arithmetic_com() {
        assert_eq!(parse_line("add"), Com::Arith("add".to_string()));
    }
    #[test]
    fn returns_label_com() {
        assert_eq!(parse_line("label MY_COOL_LABEL"), Com::Label("MY_COOL_LABEL".to_string()));
    }
    #[test]
    fn returns_if_goto_com() {
        assert_eq!(parse_line("if-goto MY_COOL_LABEL"), Com::Branch("if-goto".to_string(), "MY_COOL_LABEL".to_string()));
    }
    #[test]
    fn returns_goto_com() {
        assert_eq!(parse_line("goto MY_COOL_LABEL"), Com::Branch("goto".to_string(), "MY_COOL_LABEL".to_string()));
    }
    #[test]
    fn returns_function_com() {
        assert_eq!(parse_line("function crazy_calc.3 2"), Com::Function("crazy_calc.3".to_string(), 2));
    }
    #[test]
    fn returns_return_com() {
        assert_eq!(parse_line("return"), Com::Return);
    }
}