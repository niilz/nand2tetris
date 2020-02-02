use super::token::{ Token, TokenType, TokenStream };
use super::tokenizer::{ tokenize };

static VALID_SUBROUTINE_KEYWORDS: &[&str] = &["constructor", "function", "method"];

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

    // parse the body
    let body = build_body(&mut token_stream);
    class_tree.push_str(&body);
    
    // after body has finished, close class with closing curly brace
    let closing_curly_brace = token_stream.next().unwrap();
    class_tree.push_str(&closing_curly_brace.to_xml());

    class_tree
}

fn build_body(mut token_tail: &mut TokenStream) -> String {
    let mut body_xml = String::new();

    let next_token = token_tail.peek();
    if is_class_var_start(next_token) {
        let class_vars = compile_class_vars(token_tail, "<classVarDec>");
        body_xml.push_str(&class_vars);
    }

    if token_tail.peek().unwrap().value != "}" {
        let subroutines = compile_subroutine(&mut token_tail, "<subroutineDec>");
        body_xml.push_str(&subroutines);
    }

    body_xml
}

// Compilation Helpers
// calles itself until a semicolon (;) appears in the TokenStream
fn compile_class_vars(mut token_tail: &mut TokenStream, class_vars_xml: &str) -> String {
    let token = token_tail.next().unwrap();
    let result_class_vars_xml = class_vars_xml.to_string() + &token.to_xml();
    if token.value == ";" {
        return result_class_vars_xml + "</classVarDec>";
    }
    compile_class_vars(&mut token_tail, &result_class_vars_xml)
}

// soubroutine-compiler
fn compile_subroutine(mut token_tail: &mut TokenStream, subroutine_xml: &str) -> String {
    // Add suroutine-identifier, type and subroutine name
    let mut result_subroutine_xml: String = token_tail
                                    .take(3)
                                    .map(|token| token.to_xml())
                                    .collect();
    // Add paramlist
    result_subroutine_xml.push_str(&compile_paramlist(&mut token_tail));

    // Add subroutine-body
    //TODO result_subroutine_xml.push_str(compile_subroutine_body(...));
    
    //  Process more subroutines if available
    if VALID_SUBROUTINE_KEYWORDS.contains(&&token_tail.peek().unwrap().value.as_str()) {
        return compile_subroutine(&mut token_tail, "<subroutineDec>");
    }
    // End subroutine
    result_subroutine_xml + "</subroutineDec>"
}

// Parameterlist compiler
fn compile_paramlist(mut param_decleration: &mut TokenStream) -> String {
    let mut paramlist_xml = get_paranthese(&mut param_decleration);
    paramlist_xml.push_str("<parameterList>");
    loop {
        let token = param_decleration.next().unwrap();
        if token.value  == ")" {
            paramlist_xml.push_str("</parameterList>");
            paramlist_xml.push_str(&token.to_xml());
            return paramlist_xml;
        }
        paramlist_xml.push_str(&token.to_xml());
    }
}

// param-helper
fn get_paranthese(token_tail: &mut TokenStream) -> String {
    token_tail.next().unwrap().to_xml()
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
// allover-integration-TESTS
static dummy_code: &'static str = "\
    class Great {\
       field int x = 5;\
    }";
#[test]
#[should_panic]
fn files_without_class_keyword_panic() {
    let mock_token = Token {token_type: TokenType::Keyword, value: String::from("NOCLASS") };
    analyze_tokens(vec![mock_token]);
}
#[test]
fn vec_of_tokens_gets_compiled() {
    let mock_tokens = tokenize(dummy_code);
    let mock_result_xml = "\
        <class>\
            <keyword> class </keyword>\
                <symbol> { </symbol>\
                <identifier> Great </identifier>\
                <classVarDec>\
                    <keyword> field </keyword>\
                    <keyword> int </keyword>\
                    <identifier> x </identifier>\
                    <symbol> = </symbol>\
                    <integerConstant> 5 </integerConstant>\
                    <symbol> ; </symbol>\
                </classVarDec>\
                <symbol> } </symbol>\
        </class>";
    analyze_tokens(mock_tokens);
}

// body-builder-TESTS
#[test]
fn body_is_correct() {
    println!("{}", dummy_code);
    let dummy_tokens = tokenize("field int x;");
    let result_xml = "\
            <classVarDec>\
                <keyword> field </keyword>\
                <keyword> int </keyword>\
                <identifier> x </identifier>\
                <symbol> ; </symbol>\
            </classVarDec>\
            ";
    assert_eq!(build_body(&mut dummy_tokens.iter().peekable()), result_xml);
}

// Class-var-TESTS
#[test]
fn only_static_and_field_are_class_vars() {
    let field_keyword = Token { token_type: TokenType::Keyword, value: String::from("field") };
    let static_keyword = Token { token_type: TokenType::Keyword, value: String::from("static") };
    let no_class_var_keyword = Token { token_type: TokenType::Keyword, value: String::from("class") };
    assert_eq!(is_class_var_start(Some(&&field_keyword)), true);
    assert_eq!(is_class_var_start(Some(&&static_keyword)), true);
    assert_eq!(is_class_var_start(Some(&&no_class_var_keyword)), false);
}

#[test]
fn class_vars_compilation_returns_on_semicolon() {
    let dummy_end_of_class_var = tokenize("; function");
    assert_eq!(compile_class_vars(&mut dummy_end_of_class_var.iter().peekable(), ""), "<symbol> ; </symbol></classVarDec>");
}

#[test]
fn field_var_compiles() {
    let dummy_field = "field int num = 10;";
    let dummy_field_tokens = tokenize(dummy_field);
    let dummy_result = String::from("\
        <classVarDec>\
            <keyword> field </keyword>\
            <keyword> int </keyword>\
            <identifier> num </identifier>\
            <symbol> = </symbol>\
            <integerConstant> 10 </integerConstant>\
            <symbol> ; </symbol>\
        </classVarDec>");
    assert_eq!(compile_class_vars(&mut dummy_field_tokens.iter().peekable(), "<classVarDec>"), dummy_result);
}
#[test]
fn static_var_compiles() {
    let dummy_field = "static int num = 10;";
    let dummy_field_tokens = tokenize(dummy_field);
    let dummy_result = String::from("\
        <classVarDec>\
            <keyword> static </keyword>\
            <keyword> int </keyword>\
            <identifier> num </identifier>\
            <symbol> = </symbol>\
            <integerConstant> 10 </integerConstant>\
            <symbol> ; </symbol>\
        </classVarDec>");
    assert_eq!(compile_class_vars(&mut dummy_field_tokens.iter().peekable(), "<classVarDec>"), dummy_result);
}
#[test]
fn field_and_static_vars_compile() {
    let dummy_field = r#"static int num = 10, field boolean isValid = true;"#;
    let dummy_field_tokens = tokenize(dummy_field);
    let dummy_result = String::from("\
        <classVarDec>\
            <keyword> static </keyword>\
            <keyword> int </keyword>\
            <identifier> num </identifier>\
            <symbol> = </symbol>\
            <integerConstant> 10 </integerConstant>\
            <symbol> , </symbol>\
            <keyword> field </keyword>\
            <keyword> boolean </keyword>\
            <identifier> isValid </identifier>\
            <symbol> = </symbol>\
            <keyword> true </keyword>\
            <symbol> ; </symbol>\
        </classVarDec>");
    assert_eq!(compile_class_vars(&mut dummy_field_tokens.iter().peekable(), "<classVarDec>"), dummy_result);
}

// Subroutine-TESTS
#[test]
fn subroutine_compiled() {
    let dummy_subroutine = "\
        function void myfunc(param1, param2) {\
            
        }";
    unimplemented!();
}

// parameterList-TEST
#[test]
fn empty_parameterlist_compiles() {
    let dummy_params = tokenize("()");
    let dummy_params_xml = "\
        <symbol> ( </symbol>\
            <parameterList>\
            </parameterList>\
        <symbol> ) </symbol>";
    assert_eq!(compile_paramlist(&mut dummy_params.iter().peekable()), dummy_params_xml);
}
#[test]
fn parameterlist_compiles() {
    let dummy_params = tokenize("(int age, boolean hasHair)");
    let dummy_params_xml = "\
        <symbol> ( </symbol>\
            <parameterList>\
                <keyword> int </keyword>\
                <identifier> age </identifier>\
                <symbol> , </symbol>\
                <keyword> boolean </keyword>\
                <identifier> hasHair </identifier>\
            </parameterList>\
        <symbol> ) </symbol>";
    assert_eq!(compile_paramlist(&mut dummy_params.iter().peekable()), dummy_params_xml);
}

// If Statement-TESTS
#[test]
fn if_statement_gets_compiled() {
    let if_xml = "TEST";
    assert_eq!(compile_if(), if_xml);
}