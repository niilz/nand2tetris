use super::token::{ Token, TokenType };
use super::tokenizer::{ tokenize };

pub fn analyze_tokens(tokens: Vec<Token>) -> String {
    let mut token_stream = tokens.iter().peekable();

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

fn build_body<'a>(token_tail: &mut Iterator<Item=&Token>) -> String {
    let mut body_xml = String::new();

    let mut token_tail_peekable = token_tail.peekable();
    let next_token = token_tail_peekable.peek();
    if is_class_var_start(next_token) {
        let class_vars = compile_class_vars(&mut token_tail);
        body_xml.push_str(&class_vars);
    }
    body_xml 
}

// Compilation Helpers
fn compile_class_vars(token_tail: &mut Iterator<Item=&Token>) -> String {
    String::from("clas_var_test")
}

fn compile_if() -> String {
    let body: Vec<Token> = Vec::new();
    
    format!("<ifStatement>\n{:?}\n</ifStatement>", body)
}

// Controle-Flow-Helpers
fn is_class_var_start(maybe_token: Option<&&Token>) -> bool {
    let maybe_class_var = &maybe_token.unwrap().value;
    maybe_class_var == "static" || maybe_class_var == "field"
}



// TESTS
//
// allover-integration-tests
#[test]
#[should_panic]
fn files_without_class_keyword_panic() {
    let mock_token = Token {token_type: TokenType::Keyword, value: String::from("NOCLASS") };
    analyze_tokens(vec![mock_token]);
}

// body-builder-tests
#[test]
fn body_is_correct() {
    unimplemented!();
}

// Iterator-behavios-test
#[test]
fn iterator_gets_advanced_by_body() {
    let mock_code = "class Main { let x = y; }";
    let mock_tokens = tokenize(mock_code);
    assert_eq!(analyze_tokens(mock_tokens), "");
}


#[test]
fn if_statement_gets_compiled() {
    let if_xml = "TEST";
    assert_eq!(compile_if(), if_xml);
}