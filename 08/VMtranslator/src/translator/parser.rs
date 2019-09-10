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
        [arith] => Com::Arith(arith.to_string()),
        [cond_or_label, label] => {
            match cond_or_label {
                "label" => Com::Label(label.to_string()),
                branch_com => Com::Branch(branch_com.to_string(), label.to_string()),
            }
        }
        [pupo, segment, position] => {
            let position_option = position.parse::<u32>();
            let position = match position_option {
                Ok(value) => value,
                Err(message) => panic!("You've passed '{}'. Which could not be parsed. Error: {}", &com_fields[1], message),
            };
            match &pupo[..] {
                "push" => Com::Push(segment.to_string(), position),
                "pop" => Com::Pop(segment.to_string(), position),
                _ => panic!("Expected command to be push or pop, but the command was neither."),
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
    fn returns_branch_com() {
        assert_eq!(parse_line("if-goto MY_COOL_LABEL"), Com::Branch("if-goto".to_string(), "MY_COOL_LABEL".to_string()));
    }
}