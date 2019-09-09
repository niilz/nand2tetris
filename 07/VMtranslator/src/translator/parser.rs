#[derive(Debug)]
#[derive(PartialEq)]
// Enum with com-Variantss
pub enum Com<'a> {
    Push(&'a str, u32),
    Pop(&'a str, u32),
    Arith(&'a str),
}

// Cleanes a given line (e.g. from comments)
pub fn clean_line(line: &str) -> String {
    let trimmed = line.trim();
    if trimmed.is_empty() || &trimmed[0..2] == "//" {
        return "".to_string();
    }
    let line: Vec<&str> = trimmed.split("//").collect(); 
    line[0].trim().to_string()
}

// parses a given line -> returns a com::Variant
pub fn parse_line(line: &str) -> Option<Com> {
    let com = line.trim();
    let com_fields: Vec<&str> = com.split(" ").collect();
    let cf_len = com_fields.len();
    let arg0 = com_fields[0];
    if cf_len == 1 {
        Some(Com::Arith(arg0))
    } else if cf_len == 3 {
        let arg1 = com_fields[1];
        let arg2_option = com_fields[2].parse::<u32>();
        let arg2 = match arg2_option {
            Ok(value) => value,
            Err(message) => panic!("You've passed '{}'. Which could not be parsed. Error: {}", &com_fields[1], message),
        };
        match arg0 {
            "push" => Some(Com::Push(arg1, arg2)),
            "pop" => Some(Com::Pop(arg1, arg2)),
            _ => None,
        }
    } else {
        panic!("The given line was '{}'. It could not be parsed.", line);
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
        assert_eq!(clean_line("push 2   // some nasty comment"), "push 2".to_string());
    }
    #[test]
    fn removes_empty_line() {
        assert_eq!(clean_line("\n"), "".to_string());
    }

    // Test parse_line()
    #[test]
    fn returns_push_com() {
        assert_eq!(parse_line("push local 2"), Some(Com::Push("local", 2)));
    }
    #[test]
    fn returns_pop_com() {
        assert_eq!(parse_line("pop static 3"), Some(Com::Pop("static", 3)));
    }
    #[test]
    fn returns_arithmetic_com() {
        assert_eq!(parse_line("add"), Some(Com::Arith("add")));
    }
}