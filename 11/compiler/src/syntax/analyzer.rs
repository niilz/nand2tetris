use super::token::{ Token, TokenType, TokenStream };
use super::tokenizer::{ tokenize };
use super::tables::{ Var, ClassTable, SubroutineTable };
use std::collections::HashMap;

static OPERATORS: &[&str] = &["+", "-", "*", "/", "&", "|", "<", ">", "=", "~"];
static UNARY_OP: &[&str] = &["-", "~"];


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
    
    let class_open_curly = next_as_xml(&mut token_stream);
    class_tree.push_str(&class_open_curly);
    
    // parse the body
    let body = build_class_body(&mut token_stream, &class_name.value);
    class_tree.push_str(&body);
    
    // after body has finished, close class with closing curly brace
    let closing_curly_brace = next_as_xml(&mut token_stream);
    class_tree.push_str(&closing_curly_brace);
    
    class_tree + "</class>"
}

// Check if valid class, right in the beginning
fn is_class_var_start(maybe_token: Option<&&Token>) -> bool {
    let maybe_class_var = &maybe_token.unwrap().value;
    maybe_class_var == "static" || maybe_class_var == "field"
}

// Compilse class-body
fn build_class_body(mut token_tail: &mut TokenStream, class_name: &str) -> String {
    let mut class_table = ClassTable::default();
    let mut body_xml = String::new();

    let mut var_idx = 0;
    loop {
        let next_token = token_tail.peek();
        if !is_class_var_start(next_token) {
            break;
        }
        let class_vars = compile_class_vars(token_tail, &mut class_table, var_idx);
        var_idx += 1;
        body_xml.push_str(&class_vars);
    }

    // Add subroutines
    while token_tail.peek().unwrap().value != "}" {
        body_xml.push_str(&compile_subroutine(&mut token_tail, class_name));
    }

    body_xml
}

// Compilation Helpers
// calles itself until a semicolon (;) appears in the TokenStream
fn compile_class_vars(token_tail: &mut TokenStream, class_table: &mut ClassTable, idx: u32) -> String {
    let mut result_class_var_xml = String::from("<classVarDec>");

    let kind = token_tail.peek().unwrap().value.to_string();
    let var_kind = next_as_xml(token_tail);
    result_class_var_xml.push_str(&var_kind);
    
    let typ = token_tail.peek().unwrap().value.to_string();
    let var_type = next_as_xml(token_tail);
    result_class_var_xml.push_str(&var_type);
    
    let name = token_tail.peek().unwrap().value.to_string();
    let var_name = next_as_xml(token_tail);
    result_class_var_xml.push_str(&var_name);

    // Construct VAR
    let var = Var::new(&kind, &typ, &name, idx);
    // Add var to XML
    result_class_var_xml.push_str(&var.to_xml());
    // Add Var to Class-Table
    class_table.add(var);

    loop {
        if token_tail.peek().unwrap().value == ";" {
            break;
        }
        let comma = next_as_xml(token_tail);
        result_class_var_xml.push_str(&comma);
        let var_name = next_as_xml(token_tail);
        result_class_var_xml.push_str(&var_name);
    }
    let semicolon = token_tail.next().unwrap().to_xml();
    result_class_var_xml.push_str(&semicolon);

    result_class_var_xml + "</classVarDec>"
}

// soubroutine-compiler
fn compile_subroutine(token_tail: &mut TokenStream, class_name: &str) -> String {

    let mut subroutine_table = SubroutineTable::default();
    
    let mut result_subroutine_xml = String::from("<subroutineDec>");
    // Add suroutine-identifier, type and subroutine name
    let routine_type = token_tail.next().unwrap();
    let return_type = token_tail.next().unwrap();
    let routine_name = token_tail.next().unwrap();
    result_subroutine_xml.push_str(&routine_type.to_xml());
    result_subroutine_xml.push_str(&return_type.to_xml());
    result_subroutine_xml.push_str(&routine_name.to_xml());
    
    // TODO: only if METHOD add THIS
    if routine_type.value == "method" {
        // Create this-arg
        let this = Var::new("arg", class_name, "this", 0);
        // Add this to xml
        result_subroutine_xml.push_str(&this.to_xml());
        // Add Var to Subrroutine-Table
        subroutine_table.add(this);
    }

    let param_list = compile_paramlist(token_tail, &mut subroutine_table);
    result_subroutine_xml.push_str(&param_list);

    // Start subroutine-body
    result_subroutine_xml.push_str("<subroutineBody>");
    let subroutine_opening_curly = next_as_xml(token_tail);
    result_subroutine_xml.push_str(&subroutine_opening_curly);
    // Add code in subroutine-body
    result_subroutine_xml.push_str(&compile_subroutine_body(token_tail));

    // End subroutine
    let subroutine_closing_curly = next_as_xml(token_tail);
    result_subroutine_xml.push_str(&subroutine_closing_curly);
    result_subroutine_xml.push_str("</subroutineBody>");
    result_subroutine_xml + "</subroutineDec>"
}

// Compile PARAM (part of subroutine)
fn compile_paramlist(token_tail: &mut TokenStream, subroutine_table: &mut SubroutineTable) -> String {
    let mut paramlist_xml = next_as_xml(token_tail);
    paramlist_xml.push_str("<parameterList>");

    let mut arg_count = if subroutine_table.has_this() { 1 } else { 0 };
    loop {
        let token = token_tail.next().unwrap();
        if token.value  == ")" {
            paramlist_xml.push_str("</parameterList>");
            paramlist_xml.push_str(&token.to_xml());
            return paramlist_xml;
        }
        if ["(", ","].contains(&token.value.as_ref()) {
            // add opening paranthese or comma
            paramlist_xml.push_str(&token.to_xml());
        } else {
            // add param type and name
            let typ_token = token;
            let name_token = token_tail.next().unwrap();
            paramlist_xml.push_str(&typ_token.to_xml());
            paramlist_xml.push_str(&name_token.to_xml());
            // Create arg-var and add it to Subroutine-Table
            let arg = Var::new("arg", &typ_token.value, &name_token.value, arg_count);
            subroutine_table.add(arg);
            arg_count += 1;
            // Get updated Var and add it to XML
            let updated_var = subroutine_table.get(&name_token.value);
            paramlist_xml.push_str(&updated_var.to_xml());
        }
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

    // If closing curly appears, subroutine has no statements and can return early
    if token_tail.peek().unwrap().value == "}" {
        return result_sub_body_xml;
    }

    // add statements
    loop {
        let next_token = token_tail.peek().unwrap().value.as_str();
        if next_token == "}" || next_token == ";" {
            break;
        }
        result_sub_body_xml.push_str(&compile_statement(&mut token_tail));
    }

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
fn compile_statement(mut token_tail: &mut TokenStream) -> String {
    // concat previous statements
    let mut result_statement_xml = String::from("<statements>");
    
    loop {
        if token_tail.peek() == None {
            panic!("token_tail has no next value in compile_statement. But should either see have } of sourrounding subroutine-body or next statement or else")
        }
        
        // Check if no more subroutines?
        if token_tail.peek().unwrap().value == "}" {
            break;
        }
        
        let statement_token_value = token_tail.peek().unwrap().value.to_string();
        match statement_token_value.as_str() {
            "let" => result_statement_xml.push_str(&compile_let(&mut token_tail)),
            "if" | "while" => result_statement_xml.push_str(&compile_conditional_statement(&mut token_tail)),
            "do" => result_statement_xml.push_str(&compile_do(&mut token_tail)),
            "return" => result_statement_xml.push_str(&compile_return(&mut token_tail)),
            s => panic!("unexpected statement-keyword of: {:?}", s),
        }
    }
    
    result_statement_xml.push_str("</statements>");
    
    result_statement_xml
}

// Compile Statement body
fn compile_statement_body(mut token_tail: &mut TokenStream) -> String {
    let mut result_statement_body_xml = String::new();
    // If body is not empty, get more statements
    if token_tail.peek() == None {
        panic!("no next value available in compile_statement_body. Either } of this statement should be there or more statements");
    }
    if token_tail.peek().unwrap().value != "}" {
        result_statement_body_xml.push_str(&compile_statement(&mut token_tail));
    }
    // add closing curly of statement_body then return statement-body
    result_statement_body_xml + &next_as_xml(&mut token_tail)
}

// Compile CONDITION statement "if, while"
fn compile_conditional_statement(mut token_tail: &mut TokenStream) -> String {
    let statement_token = token_tail.next().unwrap();
    let mut result_condition_xml = format!("<{}Statement>", statement_token.value);
    // get keyword
    result_condition_xml.push_str(&statement_token.to_xml());

    // get open paranthese
    result_condition_xml.push_str(&next_as_xml(&mut token_tail));
    // Add all expressions
    result_condition_xml.push_str(&compile_expression(&mut token_tail));
    // add close paranthese
    result_condition_xml.push_str(&next_as_xml(&mut token_tail));

    // add opening curly-brace
    result_condition_xml.push_str(&next_as_xml(&mut token_tail));
    // add statement-body (includes closing curly brace)
    result_condition_xml.push_str(&compile_statement_body(&mut token_tail));

    // in case else is following the previous statement add it
    if token_tail.peek().unwrap().value == "else" {
        result_condition_xml.push_str(&compile_else(&mut token_tail));
    }

    result_condition_xml.push_str(&format!("</{}Statement>", statement_token.value));
    
    // Return ruslting condition as xml
    result_condition_xml
}

// Compile LET
fn compile_let(mut token_tail: &mut TokenStream) -> String {
    let mut result_let_xml = String::from("<letStatement>");
    // get let keyword
    result_let_xml.push_str(&next_as_xml(&mut token_tail));
    // add identifier
    result_let_xml.push_str(&next_as_xml(&mut token_tail));
    // check if array-indexing occurs
    if token_tail.peek().unwrap().value == "[" {
        // add opening square-bracket
        result_let_xml.push_str(&next_as_xml(&mut token_tail));

        // add expression inside square-brackets
        result_let_xml.push_str(&compile_expression(&mut token_tail));

        // add closing square-bracket
        result_let_xml.push_str(&next_as_xml(&mut token_tail));
    }

    // add equal sign
    result_let_xml.push_str(&next_as_xml(&mut token_tail));

    // Add Expression on right sight of assignment
    result_let_xml.push_str(&compile_expression(&mut token_tail));

    // add semicolon and return result 
    result_let_xml + &next_as_xml(&mut token_tail) + "</letStatement>"
}

// Compile DO
fn compile_do(mut token_tail: &mut TokenStream) -> String {
    
    let mut result_do_xml = String::from("<doStatement>");

    // take do, [className,.,] subroutine as xml
    result_do_xml.push_str(&consume_to_xml_while(&mut token_tail, "("));
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
    
    result_do_xml + &end_of_do + "</doStatement>"
}

// return values until condition (helper for compile_do)
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

// Compile RETURN
fn compile_return(mut token_tail: &mut TokenStream) -> String {
    
    let mut result_return_xml = String::from("<returnStatement>");
    // add return-keyword
    result_return_xml.push_str(&next_as_xml(&mut token_tail));
    // add expressions (if present)
    result_return_xml.push_str(&compile_expression(&mut token_tail));
    // add semicolon and return result
    result_return_xml + &next_as_xml(&mut token_tail) + "</returnStatement>"
}

// Compile ELSE
fn compile_else(mut token_tail: &mut TokenStream) -> String {
    let mut result_else_xml = String::new();
    // add else-keyword
    result_else_xml.push_str(&next_as_xml(&mut token_tail));
    // add opening curly
    result_else_xml.push_str(&next_as_xml(&mut token_tail));
    // add else body
    result_else_xml.push_str(&compile_statement_body(&mut token_tail));
    
    result_else_xml
}


// Compile EXPRESSION
fn compile_expression(mut token_tail: &mut TokenStream) -> String {

    if token_tail.peek() == None {
        panic!("compile_expression received a TokenStream with no next value. ")
    }
    let mut result_expression_xml = String::new();

    // If no term, just return empty String
    let next_token = token_tail.peek().unwrap().value.to_string();
    if [")", ";", "]"].contains(&next_token.as_ref())  {
        return result_expression_xml;
    }

    // start Expression
    result_expression_xml.push_str("<expression>");
    // add term
    result_expression_xml.push_str(&compile_term(&mut token_tail, ""));
    // end Expression and return
    result_expression_xml.push_str("</expression>");
    
    // add more expressions if there are any
    if token_tail.peek().unwrap().value == "," {
        result_expression_xml.push_str(&next_as_xml(&mut token_tail));
        return result_expression_xml + &compile_expression(&mut token_tail);
    }
    
    result_expression_xml
}

// Compile term
fn compile_term(mut token_tail: &mut TokenStream, result_xml: &str) -> String {
    let mut result_term_xml = result_xml.to_string();

    // add nested unary-op-exprssion if present
    if UNARY_OP.contains(&token_tail.peek().unwrap().value.as_ref()) {
        // start nested expression
        result_term_xml.push_str("<term>");
        // add unary-operator
        result_term_xml.push_str(&next_as_xml(&mut token_tail));
        // add the nested term
        result_term_xml.push_str(&compile_term(&mut token_tail, ""));
        // finish nested expression
        result_term_xml.push_str("</term>");
    }
    // add array indexing or exressions if present
    let next_token = token_tail.peek().unwrap().value.to_string();
    if ["(", "["].contains(&next_token.as_ref()) {
        result_term_xml.push_str(if next_token == "(" { "<term>" } else { "" });
        
        // add opening bracket/parantese
        result_term_xml.push_str(&next_as_xml(&mut token_tail));
        // add expression
        result_term_xml.push_str(&compile_expression(&mut token_tail));
        // add closing bracket/parantese
        result_term_xml.push_str(&next_as_xml(&mut token_tail));
        result_term_xml.push_str(if next_token == "(" { "</term>" } else { "" });
        // add more terms if there are any
        if OPERATORS.contains(&token_tail.peek().unwrap().value.as_ref()) {
            // first add the operator
            result_term_xml.push_str(&next_as_xml(&mut token_tail));
            // then add the term
            result_term_xml.push_str(&compile_term(&mut token_tail, ""));
        }
    }
    // if term is finished return it
    if [")", ";", "]", ","].contains(&token_tail.peek().unwrap().value.as_ref())  {
        return result_term_xml.to_string();
    }
    
    // start adding Expression term(s)
    result_term_xml.push_str("<term>");
    
    result_term_xml.push_str(&next_as_xml(&mut token_tail));
    // add subunits of term if present
    let next_token = token_tail.peek().unwrap().value.to_string();
    if &next_token == "." {
        result_term_xml.push_str(&consume_to_xml_while(&mut token_tail, "("));
        // add opening paranthese
        result_term_xml.push_str(&next_as_xml(&mut token_tail));
        result_term_xml.push_str("<expressionList>");
        result_term_xml.push_str(&compile_expression(&mut token_tail));
        result_term_xml.push_str("</expressionList>");
        // add closing paranthese
        result_term_xml.push_str(&next_as_xml(&mut token_tail));
    } else if next_token == "[" {
        // add indexing part of term if present
        result_term_xml.push_str(&compile_term(&mut token_tail, ""));
    }
    // end adding Expression term
    result_term_xml.push_str("</term>");

    // add op if available
    if OPERATORS.contains(&token_tail.peek().unwrap().value.as_ref()) {
        // add operator
        result_term_xml.push_str(&next_as_xml(&mut token_tail));
    }
    compile_term(&mut token_tail, &result_term_xml)
}



// #######################
// #######  TESTS  #######
// #######################
//
// allover-integration-TESTS
static dummy_code: &'static str = r#"
    class Great {
       field int x;
       static char y;
       function char myfunc(int age, boolean isCool) {
           var char letter;
           var int max, min;
           if (true) {
          }
       }
       function boolean secondFunc(char a, int 42) {
           var int size;
           if (false) {
            let x = 1;
            return "Hello";
          }
       }
    }"#;

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
                    <symbol> ; </symbol>\
                </classVarDec>\
                <classVarDec>\
                    <keyword> static </keyword>\
                    <keyword> char </keyword>\
                    <identifier> y </identifier>\
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
                                <symbol> } </symbol>\
                            </ifStatement>\
                        </statements>\
                        <symbol> } </symbol>\
                    </subroutineBody>\
                </subroutineDec>\
                <subroutineDec>\
                    <keyword> function </keyword>\
                    <keyword> boolean </keyword>\
                    <identifier> secondFunc </identifier>\
                    <symbol> ( </symbol>\
                    <parameterList>\
                        <keyword> char </keyword>\
                        <identifier> a </identifier>\
                        <symbol> , </symbol>\
                        <keyword> int </keyword>\
                        <integerConstant> 42 </integerConstant>\
                    </parameterList>\
                    <symbol> ) </symbol>\
                        <subroutineBody>\
                        <symbol> { </symbol>\
                        <varDec>\
                            <keyword> var </keyword>\
                            <keyword> int </keyword>\
                            <identifier> size </identifier>\
                            <symbol> ; </symbol>\
                        </varDec>\
                        <statements>\
                            <ifStatement>\
                                <keyword> if </keyword>\
                                <symbol> ( </symbol>\
                                    <expression>\
                                        <term>\
                                            <keyword> false </keyword>\
                                        </term>\
                                    </expression>\
                                <symbol> ) </symbol>\
                                <symbol> { </symbol>\
                                    <statements>\
                                        <letStatement>\
                                            <keyword> let </keyword>\
                                            <identifier> x </identifier>\
                                            <symbol> = </symbol>\
                                            <expression>\
                                                <term>\
                                                    <integerConstant> 1 </integerConstant>\
                                                </term>\
                                            </expression>\
                                            <symbol> ; </symbol>\
                                        </letStatement>\
                                        <returnStatement>\
                                            <keyword> return </keyword>\
                                                <expression>\
                                                    <term>\
                                                        <stringConstant> Hello </stringConstant>\
                                                    </term>\
                                                </expression>\
                                            <symbol> ; </symbol>\
                                        </returnStatement>\
                                    </statements>\
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
    assert_eq!(build_class_body(&mut dummy_tokens.iter().peekable(), "_class_name"), result_xml);
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
fn field_var_compiles() {
    let dummy_field = "field int y;";
    let dummy_field_tokens = tokenize(dummy_field);
    let dummy_result = String::from("\
        <classVarDec>\
            <keyword> field </keyword>\
            <keyword> int </keyword>\
            <identifier> y </identifier>\
            <symbol> ; </symbol>\
        </classVarDec>");
    assert_eq!(compile_class_vars(&mut dummy_field_tokens.iter().peekable(), &mut ClassTable::default(), 0), dummy_result);
}
#[test]
fn static_var_compiles() {
    let dummy_field = "static int num;";
    let dummy_field_tokens = tokenize(dummy_field);
    let dummy_result = String::from("\
        <classVarDec>\
            <keyword> static </keyword>\
            <keyword> int </keyword>\
            <identifier> num </identifier>\
            <symbol> ; </symbol>\
        </classVarDec>");
    assert_eq!(compile_class_vars(&mut dummy_field_tokens.iter().peekable(), &mut ClassTable::default(), 0), dummy_result);
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
    assert_eq!(compile_subroutine(&mut dummy_subroutine_tokens.iter().peekable(), "_class_name"), dummy_subroutine_xml);
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


// parameterList-TEST
#[test]
fn empty_parameterlist_compiles() {
    let dummy_params = tokenize("()");
    let dummy_params_xml = "\
        <symbol> ( </symbol>\
            <parameterList>\
            </parameterList>\
        <symbol> ) </symbol>";
    assert_eq!(compile_paramlist(&mut dummy_params.iter().peekable(), &mut SubroutineTable::default()), dummy_params_xml);
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
    assert_eq!(compile_paramlist(&mut dummy_params.iter().peekable(), &mut SubroutineTable::default()), dummy_params_xml);
}

// ### Statement TESTS ###

// Complex Statement body Tests
#[test]
fn statement_body_compiles() {
    let dummy_statement_body = "\
        if (true) {\
            while (false) {\
            }\
        } else {\
            do rockit();\
        }\
    }";
    let dummy_statement_body_tokens = tokenize(dummy_statement_body);
    let dummy_statement_body_xml = "\
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
                    <statements>\
                        <whileStatement>\
                            <keyword> while </keyword>\
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
                    </statements>\
                <symbol> } </symbol>\
                <keyword> else </keyword>\
                <symbol> { </symbol>\
                    <statements>\
                        <doStatement>\
                            <keyword> do </keyword>\
                            <identifier> rockit </identifier>\
                            <symbol> ( </symbol>\
                                <expressionList>\
                                </expressionList>\
                            <symbol> ) </symbol>\
                            <symbol> ; </symbol>\
                        </doStatement>\
                    </statements>\
                <symbol> } </symbol>\
                </ifStatement>\
            </statements>\
        <symbol> } </symbol>";
    assert_eq!(compile_statement_body(&mut dummy_statement_body_tokens.iter().peekable()), dummy_statement_body_xml);
}

// Let Statement-TEST
#[test]
fn let_wihtout_expression_compiles() {
    let dummy_let_tokens = tokenize("let myVar = 50;");
    let dummy_let_xml = "\
        <letStatement>\
            <keyword> let </keyword>\
            <identifier> myVar </identifier>\
            <symbol> = </symbol>\
            <expression>\
                <term>\
                    <integerConstant> 50 </integerConstant>\
                </term>\
            </expression>\
            <symbol> ; </symbol>\
        </letStatement>";
    assert_eq!(compile_let(&mut dummy_let_tokens.iter().peekable()), dummy_let_xml);
}
#[test]
fn let_with_or_compiles() {
    let dummy_let_tokens = tokenize("let myVar = 50 | 60;");
    let dummy_let_xml = "\
        <letStatement>\
            <keyword> let </keyword>\
            <identifier> myVar </identifier>\
            <symbol> = </symbol>\
            <expression>\
                <term>\
                    <integerConstant> 50 </integerConstant>\
                </term>\
                <symbol> | </symbol>\
                <term>\
                    <integerConstant> 60 </integerConstant>\
                </term>\
            </expression>\
            <symbol> ; </symbol>\
        </letStatement>";
    assert_eq!(compile_let(&mut dummy_let_tokens.iter().peekable()), dummy_let_xml);
}

#[test]
fn let_with_array_idx_compiles() {
    let dummy_let_tokens = tokenize("let myVar[i] = 50;");
    let dummy_let_xml = "\
        <letStatement>\
            <keyword> let </keyword>\
            <identifier> myVar </identifier>\
            <symbol> [ </symbol>\
              <expression>\
                <term>\
                  <identifier> i </identifier>\
                </term>\
              </expression>\
              <symbol> ] </symbol>\
            <symbol> = </symbol>\
            <expression>\
                <term>\
                    <integerConstant> 50 </integerConstant>\
                </term>\
            </expression>\
            <symbol> ; </symbol>\
        </letStatement>";
    assert_eq!(compile_let(&mut dummy_let_tokens.iter().peekable()), dummy_let_xml);
}
#[test]
fn let_subroutine_call_compiles() {
    let dummy_let_tokens = tokenize("let subR = myFunc.call();");
    let dummy_let_xml = "\
        <letStatement>\
            <keyword> let </keyword>\
            <identifier> subR </identifier>\
            <symbol> = </symbol>\
            <expression>\
                <term>\
                    <identifier> myFunc </identifier>\
                    <symbol> . </symbol>\
                    <identifier> call </identifier>\
                    <symbol> ( </symbol>\
                    <expressionList>\
                    </expressionList>\
                    <symbol> ) </symbol>\
                </term>\
            </expression>\
            <symbol> ; </symbol>\
        </letStatement>";
    assert_eq!(compile_let(&mut dummy_let_tokens.iter().peekable()), dummy_let_xml);
}
#[test]
fn let_array_idx_compiles() {
    let dummy_let_tokens = tokenize("let a[1]= blup;");
    let dummy_let_xml = "\
        <letStatement>\
            <keyword> let </keyword>\
            <identifier> a </identifier>\
            <symbol> [ </symbol>\
              <expression>\
                <term>\
                  <integerConstant> 1 </integerConstant>\
                </term>\
              </expression>\
              <symbol> ] </symbol>\
            <symbol> = </symbol>\
            <expression>\
                <term>\
                    <identifier> blup </identifier>\
                </term>\
            </expression>\
            <symbol> ; </symbol>\
        </letStatement>";
    assert_eq!(compile_let(&mut dummy_let_tokens.iter().peekable()), dummy_let_xml);
}
#[test]
fn let_with_parantheses_compiles() {
    let dummy_let_tokens = tokenize("let a = i * (-3);");
    let dummy_let_xml = "\
        <letStatement>\
            <keyword> let </keyword>\
            <identifier> a </identifier>\
            <symbol> = </symbol>\
            <expression>\
                <term>\
                  <identifier> i </identifier>\
                </term>\
                <symbol> * </symbol>\
                <term>\
                  <symbol> ( </symbol>\
                  <expression>\
                    <term>\
                      <symbol> - </symbol>\
                      <term>\
                        <integerConstant> 3 </integerConstant>\
                      </term>\
                    </term>\
                  </expression>\
                  <symbol> ) </symbol>\
                </term>\
              </expression>\
            <symbol> ; </symbol>\
        </letStatement>";
    assert_eq!(compile_let(&mut dummy_let_tokens.iter().peekable()), dummy_let_xml);
}
#[test]
fn let_with_square_term_right_compiles() {
    let dummy_let_tokens = tokenize("let sum = sum + a[i];");
    let dummy_let_xml = "\
    <letStatement>\
        <keyword> let </keyword>\
        <identifier> sum </identifier>\
        <symbol> = </symbol>\
        <expression>\
        <term>\
            <identifier> sum </identifier>\
        </term>\
        <symbol> + </symbol>\
        <term>\
            <identifier> a </identifier>\
            <symbol> [ </symbol>\
            <expression>\
            <term>\
                <identifier> i </identifier>\
            </term>\
            </expression>\
            <symbol> ] </symbol>\
        </term>\
        </expression>\
        <symbol> ; </symbol>\
    </letStatement>";
    assert_eq!(compile_let(&mut dummy_let_tokens.iter().peekable()), dummy_let_xml);
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
    }"; // Extra curly indicating that next token is end of sourrounding subroutine
    let dummy_if_while_tokens = tokenize(dummy_if_while);
    let dummy_if_while_xml = "\
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
                    <statements>\
                        <whileStatement>\
                            <keyword> while </keyword>\
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
                    </statements>\
                    <symbol> } </symbol>\
                    <keyword> else </keyword>\
                    <symbol> { </symbol>\
                    <symbol> } </symbol>\
                </ifStatement>\
            </statements>";
    assert_eq!(compile_statement(&mut dummy_if_while_tokens.iter().peekable()), dummy_if_while_xml);
}
#[test]
fn if_several_terms_compiles() {
    let dummy_if_while = "if (((y + size) < 254) & ((x + size) < 510)) {} }"; // Extra curly indicating that next token is end of sourrounding subroutine
    let dummy_if_while_tokens = tokenize(dummy_if_while);
    let dummy_if_while_xml = "\
        <statements>\
            <ifStatement>\
            <keyword> if </keyword>\
            <symbol> ( </symbol>\
            <expression>\
                <term>\
                <symbol> ( </symbol>\
                <expression>\
                    <term>\
                    <symbol> ( </symbol>\
                    <expression>\
                        <term>\
                        <identifier> y </identifier>\
                        </term>\
                        <symbol> + </symbol>\
                        <term>\
                        <identifier> size </identifier>\
                        </term>\
                    </expression>\
                    <symbol> ) </symbol>\
                    </term>\
                    <symbol> &lt; </symbol>\
                    <term>\
                    <integerConstant> 254 </integerConstant>\
                    </term>\
                </expression>\
                <symbol> ) </symbol>\
                </term>\
                <symbol> &amp; </symbol>\
                <term>\
                <symbol> ( </symbol>\
                <expression>\
                    <term>\
                    <symbol> ( </symbol>\
                    <expression>\
                        <term>\
                        <identifier> x </identifier>\
                        </term>\
                        <symbol> + </symbol>\
                        <term>\
                        <identifier> size </identifier>\
                        </term>\
                    </expression>\
                    <symbol> ) </symbol>\
                    </term>\
                    <symbol> &lt; </symbol>\
                    <term>\
                    <integerConstant> 510 </integerConstant>\
                    </term>\
                </expression>\
                <symbol> ) </symbol>\
                </term>\
            </expression>\
            <symbol> ) </symbol>\
            <symbol> { </symbol>\
            <symbol> } </symbol>\
            </ifStatement>\
        </statements>";
    assert_eq!(compile_statement(&mut dummy_if_while_tokens.iter().peekable()), dummy_if_while_xml);
}
            
// Conditionals Test (sub-piece of if and while)
#[test]
fn conditionals_compile() {
    let dummy_condition = "if (true) {}\
        }"; // sourrounding statement or subroutine end-curly
    let condition_tokens = tokenize(dummy_condition);
    let condition_xml = "\
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
            <symbol> } </symbol>\
        </ifStatement>";
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
        <doStatement>\
            <keyword> do </keyword>\
            <identifier> rockit </identifier>\
            <symbol> ( </symbol>\
                <expressionList>\
                    <expression>\
                        <term>\
                            <identifier> x </identifier>\
                        </term>\
                    </expression>\
                </expressionList>\
                <symbol> ) </symbol>\
                <symbol> ; </symbol>\
            </doStatement>";
        assert_eq!(compile_do(&mut dummy_do_tokens.iter().peekable()), dummy_do_xml);
    }
#[test]
fn do_multiple_expressions_compiles() {
    let dummy_do = "do rockit(x, y, z);";
    let dummy_do_tokens = tokenize(dummy_do);
    let dummy_do_xml = "\
        <doStatement>\
            <keyword> do </keyword>\
            <identifier> rockit </identifier>\
            <symbol> ( </symbol>\
                <expressionList>\
                    <expression>\
                        <term>\
                            <identifier> x </identifier>\
                        </term>\
                    </expression>\
                    <symbol> , </symbol>\
                    <expression>\
                        <term>\
                            <identifier> y </identifier>\
                        </term>\
                    </expression>\
                    <symbol> , </symbol>\
                    <expression>\
                        <term>\
                            <identifier> z </identifier>\
                        </term>\
                    </expression>\
                </expressionList>\
                <symbol> ) </symbol>\
                <symbol> ; </symbol>\
            </doStatement>";
        assert_eq!(compile_do(&mut dummy_do_tokens.iter().peekable()), dummy_do_xml);
    }
    
#[test]
fn do_class_dot_subroutine_compiles() {
    let dummy_do = "do Great.rockit(x);";
    let dummy_do_tokens = tokenize(dummy_do);
    let dummy_do_xml = "\
        <doStatement>\
            <keyword> do </keyword>\
                <identifier> Great </identifier>\
                <symbol> . </symbol>\
                <identifier> rockit </identifier>\
                <symbol> ( </symbol>\
                    <expressionList>\
                        <expression>\
                            <term>\
                                <identifier> x </identifier>\
                            </term>\
                        </expression>\
                    </expressionList>\
                <symbol> ) </symbol>\
            <symbol> ; </symbol>\
        </doStatement>";
    assert_eq!(compile_do(&mut dummy_do_tokens.iter().peekable()), dummy_do_xml);
}

// Return - TESTS
#[test]
fn return_without_expression_compiles() {
    let dummy_return_tokens = tokenize("return;");
    let dummy_return_xml = "\
        <returnStatement>\
            <keyword> return </keyword>\
            <symbol> ; </symbol>\
        </returnStatement>";
    assert_eq!(compile_return(&mut dummy_return_tokens.iter().peekable()), dummy_return_xml);
}
#[test]
fn return_with_expression_compiles() {
    let dummy_return_tokens = tokenize(r#"return "cool";"#);
    let dummy_return_xml = "\
        <returnStatement>\
            <keyword> return </keyword>\
            <expression>\
                <term>\
                    <stringConstant> cool </stringConstant>\
                </term>\
            </expression>\
            <symbol> ; </symbol>\
        </returnStatement>";
    assert_eq!(compile_return(&mut dummy_return_tokens.iter().peekable()), dummy_return_xml);
}

// ### Expressions TESTS ###
#[test]
fn expressionless_compiles() {
    let dummy_exp = "x;";
    let dummy_exp_tokens = tokenize(dummy_exp);
    let dummy_exp_xml = "\
       <expression>\
        <term>\
            <identifier> x </identifier>\
        </term>\
        </expression>";
    assert_eq!(compile_expression(&mut dummy_exp_tokens.iter().peekable()), dummy_exp_xml);
}

// term - TESTS (part of expression)
#[test]
fn term_compiles() {
    let dummy_term_tokens = tokenize("i / 2;");
    let dummy_term_xml = "\
        <term>\
            <identifier> i </identifier>\
        </term>\
        <symbol> / </symbol>\
        <term>\
            <integerConstant> 2 </integerConstant>\
        </term>";
    assert_eq!(compile_term(&mut dummy_term_tokens.iter().peekable(), ""), dummy_term_xml);
}
