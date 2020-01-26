use super::tokenizer::{ Token, TokenType, tokenize };

pub fn analyze_tokens(tokens: Vec<Token>) -> String {
    let mut token_stream = tokens.iter();

    let mut class_tree = String::new();

    let class_keyword = token_stream.next().unwrap();
    if class_keyword.value != "class" {
        panic!("class files need to start with class declerationr");
    }
    class_tree.push_str(&class_keyword.to_xml());

    let class_name = token_stream.next().unwrap();
    if class_name.token_type != TokenType::Identifier {
        panic!("classes need a valid Class-Identifier");
    }
    class_tree.push_str(&class_name.to_xml());
    let open_curly_brace = token_stream.next().unwrap();
    class_tree.push_str(&open_curly_brace.to_xml());

    let body = build_body(&mut token_stream);
    class_tree.push_str(&body);

    let closing_curly_brace = token_stream.next().unwrap();
    class_tree.push_str(&closing_curly_brace.to_xml());

    println!("CLASS-TREE HERE: {}", class_tree);
    String::from("not yet implemented")
}

fn build_body<'a>(token_tail: &mut Iterator<Item=&'a Token>) -> String {
    token_tail.next();
    token_tail.next();
    token_tail.next();
    token_tail.next();
    token_tail.next();
    String::from("test")
}

// Compilation Helpers
fn compile_if() -> String {
    let body: Vec<Token> = Vec::new();

    format!("<ifStatement>\n{:?}\n</ifStatement>", body)
}


// TESTS

#[test]
fn iterator_gets_advanced_by_body() {
    let mock_code = "class Main { let x = y; }";
    let mock_tokens = tokenize(mock_code);
    assert_eq!(analyze_tokens(mock_tokens), "");
}

#[test]
#[should_panic]
fn files_without_class_keyword_panic() {
    let mock_token = Token {token_type: TokenType::Keyword, value: String::from("NOCLASS") };
    analyze_tokens(vec![mock_token]);
}

#[test]
fn if_statement_gets_compiled() {
    let if_xml = "TEST";
    assert_eq!(compile_if(), if_xml);
}