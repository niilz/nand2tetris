use super::token::{ Token, TokenType, TokenStream };
use super::tokenizer::{ tokenize };

static VALID_SUBROUTINE_KEYWORDS: &[&str] = &["constructor", "function", "method"];
static VALID_STATEMENT_KEYWORDS: &[&str] = &["let", "if", "else", "while", "do", "return"];

pub fn analyze_tokens(tokens: Vec<Token>) -> String {
    let mut token_stream = tokens.iter().peekable();
    
    
    let class_keyword = token_stream.next().unwrap();
    if class_keyword.value != "class" {
        panic!("class files need to start with class declerationr");
    }
    
    let mut class_tree = String::from("<class>");
    class_tree.push_str(&class_keyword.to_xml());
    
    let class_name = token_stream.next().unwrap();
    if class_name.token_type != TokenType::Identifier {
        panic!("classes need a valid Class-Identifier");
    }
    class_tree.push_str(&class_name.to_xml());
    
    let class_open_curly = token_stream.next().unwrap();
    class_tree.push_str(&class_open_curly.to_xml());
    
    // parse the body
    let body = build_class_body(&mut token_stream);
    class_tree.push_str(&body);
    
    // after body has finished, close class with closing curly brace
    let closing_curly_brace = token_stream.next().unwrap();
    class_tree.push_str(&closing_curly_brace.to_xml());
    
    class_tree + "</class>"
}

// Check if valid class, right in the beginning
fn is_class_var_start(maybe_token: Option<&&Token>) -> bool {
    let maybe_class_var = &maybe_token.unwrap().value;
    maybe_class_var == "static" || maybe_class_var == "field"
}

fn build_class_body(mut token_tail: &mut TokenStream) -> String {
    let mut body_xml = String::new();

    let next_token = token_tail.peek();
    if is_class_var_start(next_token) {
        let class_vars = compile_class_vars(token_tail, "<classVarDec>");
        body_xml.push_str(&class_vars);
    }

    // (Note: maybe looping)
    if token_tail.peek().unwrap().value != "}" {
        let subroutines = compile_subroutine(&mut token_tail, "");
        body_xml.push_str(&subroutines);
    }

    // TODO: more subroutines?

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
    let mut result_subroutine_xml = String::from("<subroutineDec>");
    // Add suroutine-identifier, type and subroutine name
    token_tail
        .take(3)
        .map(|t| {println!("TOKEN {:?}", t); t})
        .for_each(|token| result_subroutine_xml.push_str(&token.to_xml()));
    
    let param_list = compile_paramlist(&mut token_tail);
    result_subroutine_xml.push_str(&param_list);

    // Start subroutine-body
    result_subroutine_xml.push_str("<subroutineBody>");
    let subroutine_opening_curly = token_tail.next().unwrap();
    result_subroutine_xml.push_str(&subroutine_opening_curly.to_xml());
    // Add code in subroutine-body
    result_subroutine_xml.push_str(&compile_subroutine_body(&mut token_tail));

    // End subroutine
    let subroutine_closing_curly = token_tail.next().unwrap();
    result_subroutine_xml.push_str(&subroutine_closing_curly.to_xml());
    result_subroutine_xml.push_str("</subroutineBody>");
    result_subroutine_xml + "</subroutineDec>"
}

// Compile PARAM (part of subroutine)
fn compile_paramlist(mut param_decleration: &mut TokenStream) -> String {
    let mut paramlist_xml = next_as_xml(&mut param_decleration);
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
fn next_as_xml(token_tail: &mut TokenStream) -> String {
    token_tail.next().unwrap().to_xml()
}

// Compile SOUBROUTINE-BODY (part of subroutine)
fn compile_subroutine_body(mut token_tail: &mut TokenStream) -> String {
    let mut result_sub_body_xml = String::new();
    // add var-decleration if there are any
    loop {
        let next_token = token_tail.peek().unwrap();
        if next_token.value != "var" {
            break;
        }
        result_sub_body_xml.push_str(&compile_var_dec(&mut token_tail));
    }
    // TODO: compile_statements (let, if, else, while, do, return)
    // start Statements
    result_sub_body_xml.push_str("<statements>");
    
    // single statement (NOTE: maybe loop)
    result_sub_body_xml.push_str(&compile_statement(&mut token_tail, ""));

    // end Statements
    result_sub_body_xml.push_str("</statements>");
    result_sub_body_xml
}
// Compile VAR-DECLERATION (part of subroutine-body)
fn compile_var_dec(token_tail: &mut TokenStream) -> String {
    let mut var_dec_xml = String::from("<varDec>");
    loop {
        let next_token = token_tail.next().unwrap();
        if next_token.value == ";" {
            var_dec_xml.push_str(&next_token.to_xml());
            return var_dec_xml + "</varDec>";
        }
        var_dec_xml.push_str(&next_token.to_xml());
    }
}

// ### Statements TESTS ###

// Compile STATEMENTS
fn compile_statement(mut token_tail: &mut TokenStream, statements_xml: &str) -> String {
    let statement_token = token_tail.next().unwrap();
    // concat previous statements
    let mut result_statement_xml = format!("{}<{}Statement>", statements_xml, statement_token.value);
    
    match statement_token.value.as_str() {
        // "let" => result_statement_xml.push_str(&compile_let(&mut token_tail)),
        "if" | "while" => result_statement_xml.push_str(&compile_conditional_statement(&mut token_tail)),
        "else" => result_statement_xml.push_str(&compile_statement_body(&mut token_tail)),
        "do" => result_statement_xml.push_str(&compile_do(&mut token_tail)),
        // "return" => result_statement_xml.push_str(compile_return(&mut token_tail)),
        s => panic!("unexpected statement of: {:?}", s),
    }
    result_statement_xml.push_str(&format!("</{}Statement>", statement_token.value));

    if token_tail.peek().unwrap().value != "}" {
        return compile_statement(&mut token_tail, &result_statement_xml);
    }

    result_statement_xml
}


// Compile CONDITION statement (if, while)
fn compile_conditional_statement(mut token_tail: &mut TokenStream) -> String {
    let mut result_condition_xml = next_as_xml(&mut token_tail);
    // start Expression
    result_condition_xml.push_str("<expression>");
    // Expression terms
    result_condition_xml.push_str(&compile_expression(&mut token_tail));
    // end Expression
    result_condition_xml.push_str("</expression>");
    result_condition_xml.push_str(&next_as_xml(&mut token_tail));
    
    // start statement-body
    result_condition_xml + &compile_statement_body(&mut token_tail)
}

// Compile Statement body
fn compile_statement_body(mut token_tail: &mut TokenStream) -> String {
    // get opening curly first
    let mut result_statement_body_xml = next_as_xml(&mut token_tail);
    // If body is not empty, get more statements
    if token_tail.peek().unwrap().value != "}" {
        result_statement_body_xml.push_str(&compile_statement(&mut token_tail, ""));
    }
    // get closing curly to end statement-body
    result_statement_body_xml + &next_as_xml(&mut token_tail)

}

// Compile DO
fn compile_do(mut token_tail: &mut TokenStream) -> String {
    // take do, [className,.,] subroutine as xml
    let mut result_do_xml = consume_to_xml_while(&mut token_tail, "(");
    // add opening expression-list paranthese
    result_do_xml.push_str(&next_as_xml(&mut token_tail));
    // add expression-list
    result_do_xml.push_str("<expressionList>");
    result_do_xml.push_str(&compile_expression(&mut token_tail));
    result_do_xml.push_str("</expressionList>");
    // add closing expression-list paranthese and semicolon
    let end_of_do: String = token_tail
            .take(2)
            .map(|token| token.to_xml())
            .collect();
    
    result_do_xml + &end_of_do
}
// return values until condition
fn consume_to_xml_while(token_tail: &mut TokenStream, break_marker: &str) -> String {
    let mut result_xml = String::new();
    loop {
        let next_token = token_tail.peek().unwrap();
        if next_token.value == break_marker {
            return result_xml;
        }
        result_xml.push_str(&token_tail.next().unwrap().to_xml())
    }
}

// Compile EXPRESSION
fn compile_expression(token_tail: &mut TokenStream) -> String {
    let result_expression = String::from("<term>");
    // TODO: multiple terms (maybe: fn compile_term)
    let term = token_tail.next().unwrap().to_xml();
    result_expression + &term + "</term>"
}





// #######################
// #######  TESTS  #######
// #######################
//
// allover-integration-TESTS
static dummy_code: &'static str = "\
    class Great {\
       field int x = 5;\
       function char myfunc(int age, boolean isCool) {\
           var char letter;\
           var int max, min;\
           if (true) {\
          }
       }\
    }";

// ### CLASS - TESTS
#[test]
#[should_panic]
fn files_without_class_keyword_panic() {
    let mock_token = Token {token_type: TokenType::Keyword, value: String::from("NOCLASS") };
    analyze_tokens(vec![mock_token]);
}
// All-in-one - TEST
#[test]
fn vec_of_tokens_gets_compiled() {
    let mock_tokens = tokenize(dummy_code);
    let mock_result_xml = "\
        <class>\
            <keyword> class </keyword>\
            <identifier> Great </identifier>\
            <symbol> { </symbol>\
                <classVarDec>\
                    <keyword> field </keyword>\
                    <keyword> int </keyword>\
                    <identifier> x </identifier>\
                    <symbol> = </symbol>\
                    <integerConstant> 5 </integerConstant>\
                    <symbol> ; </symbol>\
                </classVarDec>\
                <subroutineDec>\
                    <keyword> function </keyword>\
                    <keyword> char </keyword>\
                    <identifier> myfunc </identifier>\
                    <symbol> ( </symbol>\
                    <parameterList>\
                        <keyword> int </keyword>\
                        <identifier> age </identifier>\
                        <symbol> , </symbol>\
                        <keyword> boolean </keyword>\
                        <identifier> isCool </identifier>\
                    </parameterList>\
                    <symbol> ) </symbol>\
                        <subroutineBody>\
                        <symbol> { </symbol>\
                        <varDec>\
                            <keyword> var </keyword>\
                            <keyword> char </keyword>\
                            <identifier> letter </identifier>\
                            <symbol> ; </symbol>\
                        </varDec>\
                        <varDec>\
                            <keyword> var </keyword>\
                            <keyword> int </keyword>\
                            <identifier> max </identifier>\
                            <symbol> , </symbol>\
                            <identifier> min </identifier>\
                            <symbol> ; </symbol>\
                        </varDec>\
                        <statements>\
                            <ifStatement>\
                                <keyword> if </keyword>\
                                <symbol> ( </symbol>\
                                    <expression>\
                                        <term>\
                                            <keyword> true </keyword>\
                                        </term>\
                                    </expression>\
                                <symbol> ) </symbol>\
                                <symbol> { </symbol>\
                                <TODO:  MORE  STATEMENTS>
                                <symbol> } </symbol>\
                            </ifStatement>\
                        </statements>\
                        <symbol> } </symbol>\
                    </subroutineBody>\
                </subroutineDec>\
                <symbol> } </symbol>\
        </class>";
    assert_eq!(analyze_tokens(mock_tokens), mock_result_xml);
}

// class-body-builder-TESTS
#[test]
fn class_body_is_correct() {
    let dummy_tokens = tokenize("field int x; }");
    let result_xml = "\
            <classVarDec>\
                <keyword> field </keyword>\
                <keyword> int </keyword>\
                <identifier> x </identifier>\
                <symbol> ; </symbol>\
            </classVarDec>\
            ";
    assert_eq!(build_class_body(&mut dummy_tokens.iter().peekable()), result_xml);
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

// ### Subroutine-TESTS ###
#[test]
fn subroutine_compiled() {
    let dummy_subroutine = "\
        function char myfunc(int age, boolean isCool) {\
            var char letter;\
            var int max, min;\
        }";
    let dummy_subroutine_tokens = tokenize(dummy_subroutine);
    let dummy_subroutine_xml = "\
        <subroutineDec>\
            <keyword> function </keyword>\
            <keyword> char </keyword>\
            <identifier> myfunc </identifier>\
            <symbol> ( </symbol>\
            <parameterList>\
                <keyword> int </keyword>\
                <identifier> age </identifier>\
                <symbol> , </symbol>\
                <keyword> boolean </keyword>\
                <identifier> isCool </identifier>\
            </parameterList>\
            <symbol> ) </symbol>\
                <subroutineBody>\
                <symbol> { </symbol>\
                <varDec>\
                    <keyword> var </keyword>\
                    <keyword> char </keyword>\
                    <identifier> letter </identifier>\
                    <symbol> ; </symbol>\
                </varDec>\
                <varDec>\
                    <keyword> var </keyword>\
                    <keyword> int </keyword>\
                    <identifier> max </identifier>\
                    <symbol> , </symbol>\
                    <identifier> min </identifier>\
                    <symbol> ; </symbol>\
                </varDec>\
                <symbol> } </symbol>\
            </subroutineBody>\
        </subroutineDec>";
    assert_eq!(compile_subroutine(&mut dummy_subroutine_tokens.iter().peekable(), ""), dummy_subroutine_xml);
}

// Subroutine-Body-TESTS
#[test]
fn subroutine_body_compiles() {
    let mock_body = "\
        var int num, count;\
        var boolean isOpen;\
    }";
    // TODO: add statments
    let mock_body_tokens = tokenize(mock_body);
    let mock_body_xml = "\
        <varDec>\
            <keyword> var </keyword>\
            <keyword> int </keyword>\
            <identifier> num </identifier>\
            <symbol> , </symbol>\
            <identifier> count </identifier>\
            <symbol> ; </symbol>\
        </varDec>\
        <varDec>\
            <keyword> var </keyword>\
            <keyword> boolean </keyword>\
            <identifier> isOpen </identifier>\
            <symbol> ; </symbol>\
        </varDec>";
    assert_eq!(compile_subroutine_body(&mut mock_body_tokens.iter().peekable()), mock_body_xml);
}

fn subroutine_vars_compile() {
    let mock_body = "\
        var int num, count;\
        var boolean isOpen;";
    let mock_body_xml = "\
            <varDec>\
                <keyword> var </keyword>\
                <keyword> int </keyword>\
                <identifier> num </identifier>\
                <symbol> , </symbol>\
                <identifier> count </identifier>\
                <keyword> var </keyword>\
                <keyword> boolean </keyword>\
                <identifier> isOpen </identifier>\
                <symbol> ; </symbol>\
            </varDec>";
    let mock_body_tokens = tokenize(mock_body);
    assert_eq!(compile_var_dec(&mut mock_body_tokens.iter().peekable()), mock_body_xml);
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

// ### Statement TESTS ###

// Complex Statement body Tests
#[test]
fn statement_body_compiles() {
    let dummy_statement_body = "{\
            if (true) {\
                while (false) {\
                }\
            } else {\
                do rockit();\
            }\
        }";
    let dummy_statement_body_tokens = tokenize(dummy_statement_body);
    let dummy_statement_body_xml = "\
        <symbol> { </symbol>\
            <ifStatement>\
                <symbol> ( </symbol>\
                    <expression>\
                        <term>\
                            <keyword> true </keyword>\
                        </term>\
                    </expression>\
                <symbol> ) </symbol>\
                <symbol> { </symbol>\
                    <whileStatement>\
                        <symbol> ( </symbol>\
                            <expression>\
                                <term>\
                                    <keyword> false </keyword>\
                                </term>\
                            </expression>\
                        <symbol> ) </symbol>\
                        <symbol> { </symbol>\
                        <symbol> } </symbol>\
                    </whileStatement>\
                <symbol> } </symbol>\
            </ifStatement>\
            <elseStatement>\
                <symbol> { </symbol>\
                    <doStatement>\
                        <keyword> do </keyword>\
                        <identifier> rockit </identifier>\
                        <symbol> ( </symbol>\
                            <expressionList>\
                            </expressionlist>\
                        <symbol> ) </symbol>\
                        <symbol> ; </symbol>\
                    </doStatement>\
                <symbol> } </symbol>\
            </elseStatement>\
        <symbol> } </symbol>";
    assert_eq!(compile_statement_body(&mut dummy_statement_body_tokens.iter().peekable()), dummy_statement_body_xml);
}

// If Statement-TEST
#[test]
fn if_else_while_statements_compile() {
    let dummy_if_while = "\
        if (true) {\
            while (false) {\
            }\
        } else {\
        }\
    }"; // extra bracket to signal caller (compile_statement) -> statement is done
    let dummy_if_while_tokens = tokenize(dummy_if_while);
    let dummy_if_while_xml = "\
        <ifStatement>\
            <symbol> ( </symbol>\
                <expression>\
                    <term>\
                        <keyword> true </keyword>\
                    </term>\
                </expression>\
            <symbol> ) </symbol>\
            <symbol> { </symbol>\
                <whileStatement>\
                    <symbol> ( </symbol>\
                        <expression>\
                            <term>\
                                <keyword> false </keyword>\
                            </term>\
                        </expression>\
                    <symbol> ) </symbol>\
                    <symbol> { </symbol>\
                    <symbol> } </symbol>\
                </whileStatement>\
                <symbol> } </symbol>\
                </ifStatement>\
                <elseStatement>\
                    <symbol> { </symbol>\
                    <symbol> } </symbol>\
                </elseStatement>";
    assert_eq!(compile_statement(&mut dummy_if_while_tokens.iter().peekable(), ""), dummy_if_while_xml);
}
            
// Conditionals Test (sub-piece of if and while)
#[test]
fn conditionals_compile() {
    let dummy_condition = "(true) {}";
    let condition_tokens = tokenize(dummy_condition);
    let condition_xml = "\
        <symbol> ( </symbol>\
            <expression>\
                <term>\
                    <keyword> true </keyword>\
                </term>\
            </expression>\
        <symbol> ) </symbol>\
        <symbol> { </symbol>\
        <symbol> } </symbol>";
    assert_eq!(compile_conditional_statement(&mut condition_tokens.iter().peekable()), condition_xml);
}

// Do - Test
#[test]
fn consume_to_xml_works() {
    let dummy_tokens = tokenize("do ClassName.run(x)");
    let dummy_xml = "\
        <keyword> do </keyword>\
        <identifier> ClassName </identifier>\
        <symbol> . </symbol>\
        <identifier> run </identifier>\
        <symbol> ( </symbol>\
            <identifier> x </identifier>";
    assert_eq!(consume_to_xml_while(&mut dummy_tokens.iter().peekable(), ")"), dummy_xml);
}
#[test]
fn do_subroutine_compiles() {
    let dummy_do = "do rockit(x);";
    let dummy_do_tokens = tokenize(dummy_do);
    let dummy_do_xml = "\
    <keyword> do </keyword>\
    <identifier> rockit </identifier>\
    <symbol> ( </symbol>\
        <expressionList>\
            <term>\
                <identifier> x </identifier>\
            </term>\
        </expressionList>\
        <symbol> ) </symbol>\
        <symbol> ; </symbol>\
        ";
        assert_eq!(compile_do(&mut dummy_do_tokens.iter().peekable()), dummy_do_xml);
    }
    #[test]
    fn do_class_dot_subroutine_compiles() {
    let dummy_do = "do Great.rockit(x);";
    let dummy_do_tokens = tokenize(dummy_do);
    let dummy_do_xml = "\
        <keyword> do </keyword>\
            <identifier> Great </identifier>\
            <symbol> . </symbol>\
            <identifier> rockit </identifier>\
            <symbol> ( </symbol>\
                <expressionList>\
                    <term>\
                        <identifier> x </identifier>\
                    </term>\
                </expressionList>\
            <symbol> ) </symbol>\
        <symbol> ; </symbol>\
    ";
    assert_eq!(compile_do(&mut dummy_do_tokens.iter().peekable()), dummy_do_xml);
}

// ### Expressions TESTS ###
#[test]
fn expressionless_compiles() {
    // TODO: propper expressions
    let dummy_exp = "x";
    let dummy_exp_tokens = tokenize(dummy_exp);
    let dummy_exp_xml = "\
        <term>\
            <identifier> x </identifier>\
        </term>";
    assert_eq!(compile_expression(&mut dummy_exp_tokens.iter().peekable()), dummy_exp_xml);
}
