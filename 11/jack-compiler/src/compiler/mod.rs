pub mod tables;
pub mod code_writer;

use crate::tokenizer::token::{ Token, TokenType, TokenStream };
use crate::tokenizer::{ tokenize };
use tables::{ Var, ClassTable, SubroutineTable };
use code_writer::*;

static OPERATORS: &[&str] = &["+", "-", "*", "/", "&", "|", "<", ">", "=", "~"];
static UNARY_OP: &[&str] = &["-", "~"];


pub fn analyze_tokens(tokens: Vec<Token>) -> Vec<String> {
    let mut token_stream = tokens.iter().peekable();
    
    let class_keyword = token_stream.next().unwrap();
    if class_keyword.value != "class" {
        panic!("class files need to start with class decleration");
    }
    
    
    let class_name = token_stream.next().unwrap();
    if class_name.token_type != TokenType::Identifier {
        panic!("classes need a valid Class-Identifier");
    }
    
    let mut class_byte_code = Vec::new();
    class_byte_code.push(format!("// ByteCode for class '{}'\n", class_name.value));

    // skip opening curly-brace
    token_stream.next();
    
    // parse the body
    let body = build_class_body(&mut token_stream, &class_name.value);
    class_byte_code.extend(body);
    
    // after body has finished, check for closing curly
    let closing_curly = &token_stream.next().unwrap().value;
    if closing_curly != "}" {
        panic!("Class-Files should end with closing curly bracket, but token was '{}'", closing_curly);
    };

    match token_stream.next() {
        None => class_byte_code,
        Some(token) => panic!("Expected eof, but got token {:?}", token), 
    }
}

// Check if valid class, right in the beginning
fn is_class_var_start(maybe_token: Option<&&Token>) -> bool {
    let maybe_class_var = &maybe_token.unwrap().value;
    maybe_class_var == "static" || maybe_class_var == "field"
}

// Compilse class-body
fn build_class_body(mut token_tail: &mut TokenStream, class_name: &str) -> Vec<String> {
    let mut class_table = ClassTable::default();
    let mut class_body_byte_code = Vec::new();

    loop {
        let next_token = token_tail.peek();
        if !is_class_var_start(next_token) {
            break;
        }
        // Has no output, but registers the class-vars
        compile_class_vars(token_tail, &mut class_table);
        class_body_byte_code.push(
            format!("// Class-Var-Dec Fields: #{}, Statics: #{}\n",
                class_table.get_next_idx("field"), class_table.get_next_idx("static")));
    }

    // Add subroutines
    while token_tail.peek().unwrap().value != "}" {
        class_body_byte_code.extend(compile_subroutine(&mut token_tail, class_name, &class_table));
    }
    class_body_byte_code
}

// Compilation Helpers
// calles itself until a semicolon (;) appears in the TokenStream
// Registers Class-Vars in ClassVarTable
fn compile_class_vars(token_tail: &mut TokenStream, class_table: &mut ClassTable) {

    let var_kind = &token_tail.next().unwrap().value;
    let var_typ = &token_tail.next().unwrap().value;
    let var_name = &token_tail.next().unwrap().value;

    // Construct VAR
    let var = Var::new(&var_kind, &var_typ, class_table.get_next_idx(&var_kind));
    // Add Var to Class-Table
    class_table.add(&var_name, var);

    loop {
        if token_tail.peek().unwrap().value == ";" {
            break;
        }
        // ignore Comma
        token_tail.next();
        let var_token = token_tail.next().unwrap();
        // Create Var and add data to xml
        let var = Var::new(&var_kind, &var_typ, class_table.get_next_idx(&var_kind));
        // Add to Class-Table
        class_table.add(&var_token.value, var);
    }
    // ignore Semicolon (end of decleration)
    let semicolon = &token_tail.next().unwrap().value;
    if semicolon != ";" {
        panic!("Semicolon expected as end of ClassVarDec, but '{:?}' was found.", semicolon);
    }
    token_tail.next();
}
fn lookup(var: &Token, class_table: &ClassTable, subroutine_table: &SubroutineTable) -> Var {
    match var.token_type {
        TokenType::IntegerConstant => {
            let value = var.value.parse::<u32>().unwrap();
            Var::new("constant", "_", value)
        },
        TokenType::Identifier => {
            match subroutine_table.get(&var.value) {
                Some(var) => var,
                None => match class_table.get(&var.value) {
                    Some(var) => var,
                    None => panic!("Variable '{:?}' has not been declared.", var.value),
                }
            }
        },
        _ => panic!("Lookup for token-type '{:?}' is not implemented", var.token_type),
    }
}

// soubroutine-compiler
fn compile_subroutine(token_tail: &mut TokenStream, class_name: &str, class_table: &ClassTable) -> Vec<String> {

    let mut subroutine_table = SubroutineTable::default();
    
    let mut subroutine_byte_code = Vec::new();
    // Add suroutine-keyword, type and subroutine name/identifier
    let routine_keyword = token_tail.next().unwrap();
    let _return_type = token_tail.next().unwrap();
    let routine_name = token_tail.next().unwrap();
    
    if routine_keyword.value == "method" {
        // Create this-arg
        let this = Var::new("arg", class_name, 0);
        // Add Var to Subrroutine-Table
        subroutine_table.add("this", this);
    }
    // Add parameters to Subroutine-Table (no code-creation)
    let _param_list = compile_paramlist(token_tail, &mut subroutine_table);
    subroutine_byte_code.push(format!("function {}.{} {}\n", class_name, routine_name.value, subroutine_table.get_next_idx("arg")));

    // Ignore opening curly-brace
    token_tail.next();
    // Start subroutine-body
    subroutine_byte_code.push(String::from("// SubroutineBody START"));
    // Add code in subroutine-body
    subroutine_byte_code.extend(compile_subroutine_body(token_tail, class_table, &mut subroutine_table));
    subroutine_byte_code.push(String::from("// SubroutineBody END"));

    // End subroutine
    let closing_curly = token_tail.next().unwrap();
    if closing_curly.value != "}" {
        panic!("Expected closing curly at end of SubroutineBody, but got '{:?}'", closing_curly);
    }
    subroutine_byte_code
}

// Compile PARAM (part of subroutine)
fn compile_paramlist(token_tail: &mut TokenStream, subroutine_table: &mut SubroutineTable) {

    loop {
        let token = token_tail.next().unwrap();

        if token.value  == ")" { break; }

        // If opening paranthese -> ignore it
        if ["(", ","].contains(&token.value.as_ref()) { continue; }

        // add param type and name
        let typ_token = token;
        let name_token = token_tail.next().unwrap();
        // Create arg-var and add it to Subroutine-Table
        let arg = Var::new("arg", &typ_token.value, subroutine_table.get_next_idx("arg"));
        // Add arg to Subroutine-Table
        subroutine_table.add(&name_token.value, arg);
    }
}
// param-helper
fn next_as_xml(token_tail: &mut TokenStream) -> String {
    token_tail.next().unwrap().to_xml()
}

// Compile SOUBROUTINE-BODY (part of subroutine)
fn compile_subroutine_body(token_tail: &mut TokenStream, class_table: &ClassTable, subroutine_table: &mut SubroutineTable) -> Vec<String> {
    let mut subroutine_body_byte_code = Vec::new();
    // add var-decleration if there are any
    loop {
        let next_token = token_tail.peek().unwrap();
        if next_token.value != "var" {
            break;
        }
        // Registers local Vars (no byte code)
        compile_var_dec(token_tail, subroutine_table);
        subroutine_body_byte_code.push(format!("// Local-Var-Dec Locals: #{}\n", subroutine_table.get_next_idx("local")));
    }

    // If closing curly appears, subroutine has no statements and can return early
    if token_tail.peek().unwrap().value == "}" {
        return subroutine_body_byte_code;
    }

    // add statements
    loop {
        let next_token = token_tail.peek().unwrap().value.as_str();
        if next_token == "}" || next_token == ";" {
            break;
        }
        subroutine_body_byte_code.push(String::from("// Subroutine-Statements START"));
        subroutine_body_byte_code.extend(compile_statement(token_tail, class_table, subroutine_table));
        subroutine_body_byte_code.push(String::from("// Subroutine-Statements END"));
    }
    subroutine_body_byte_code
}

// Compile VAR-DECLERATION (part of subroutine-body)
fn compile_var_dec(token_tail: &mut TokenStream, subroutine_table: &mut SubroutineTable) {
    let var_keyword_token = token_tail.next().unwrap();
    let var_type_token = token_tail.next().unwrap();
    let var_name_token = token_tail.next().unwrap();
    
    // Construct first Var and add it to Subroutine-Table
    let var = Var::new("local", &var_type_token.value, subroutine_table.get_next_idx("local"));
    subroutine_table.add(&var_name_token.value, var);

    loop {
        let next_token = token_tail.next().unwrap();
        // If no more Local-Var decs -> quit
        if next_token.value == ";" { break; }
        // Otherwise register more Var(s) of same type (ignore the comma)
        if next_token.value == "," {
            let next_var_name = token_tail.next().unwrap();
            // construct var behind comma and add it to Subroutine-Table
            let var = Var::new("local", &var_type_token.value, subroutine_table.get_next_idx("local"));
            subroutine_table.add(&next_var_name.value, var);
        }
    }
}

// Compile STATEMENTS
fn compile_statement(token_tail: &mut TokenStream, class_table: &ClassTable, subroutine_table: &SubroutineTable) -> Vec<String> {
    let mut statement_byte_code = Vec::new();
    
    let mut statement_count = 1;
    loop {
        if token_tail.peek() == None {
            panic!("token_tail has no next value in compile_statement. But should either see have } of sourrounding subroutine-body or next statement or else")
        }
        // Check if no more subroutines?
        if token_tail.peek().unwrap().value == "}" {
            break;
        }
        let statement_token_value = token_tail.peek().unwrap().value.to_string();
        statement_byte_code.push(format!("// Statement Nr.{} START", statement_count));
        match statement_token_value.as_str() {
            "let" => statement_byte_code.extend(compile_let(token_tail, class_table, subroutine_table)),
            //"if" | "while" => result_statement_xml.push_str(&compile_conditional_statement(token_tail, class_table, subroutine_table)),
            "do" => statement_byte_code.extend(compile_do(token_tail, class_table, subroutine_table)),
            "return" => statement_byte_code.extend(compile_return(token_tail, class_table, subroutine_table)),
            s => panic!("unexpected statement-keyword of: {:?}", s),
        }
        statement_byte_code.push(format!("// Statement Nr.{} END", statement_count));
        statement_count += 1;
    }
    statement_byte_code
}

// Compile Statement body
fn compile_statement_body(token_tail: &mut TokenStream, class_table: &ClassTable, subroutine_table: &SubroutineTable) -> Vec<String> {
    let mut statement_body_byte_code = Vec::new();
    // If body is not empty, get more statements
    if token_tail.peek() == None {
        panic!("no next value available in compile_statement_body. Either } of this statement should be there or more statements");
    }
    if token_tail.peek().unwrap().value != "}" {
        statement_body_byte_code.extend(compile_statement(token_tail, class_table, subroutine_table));
    }
    // Check for closing curly and then ignore it
    let closing_curly = token_tail.next().unwrap();
    if closing_curly.value != "}" {
        panic!("End of StatementBody expects closing }} but got '{:?}' ", closing_curly);
    }
    statement_body_byte_code
}

// Compile CONDITION statement "if, while"
// fn compile_conditional_statement(token_tail: &mut TokenStream, class_table: &ClassTable, subroutine_table: &SubroutineTable) -> String {
//     let statement_token = token_tail.next().unwrap();
//     let mut result_condition_xml = format!("<{}Statement>", statement_token.value);
//     // get keyword
//     result_condition_xml.push_str(&statement_token.to_xml());

//     // get open paranthese
//     result_condition_xml.push_str(&next_as_xml(token_tail));
//     // Add all expressions
//     result_condition_xml.push_str(&compile_expression(token_tail, class_table, subroutine_table));
//     // add close paranthese
//     result_condition_xml.push_str(&next_as_xml(token_tail));

//     // add opening curly-brace
//     result_condition_xml.push_str(&next_as_xml(token_tail));
//     // add statement-body (includes closing curly brace)
//     result_condition_xml.push_str(&compile_statement_body(token_tail, class_table, subroutine_table));

//     // in case else is following the previous statement add it
//     if token_tail.peek().unwrap().value == "else" {
//         result_condition_xml.push_str(&compile_else(token_tail, class_table, subroutine_table));
//     }

//     result_condition_xml.push_str(&format!("</{}Statement>", statement_token.value));
    
//     // Return ruslting condition as xml
//     result_condition_xml
// }

// Compile LET
pub fn compile_let(token_tail: &mut TokenStream, class_table: &ClassTable, subroutine_table: &SubroutineTable) -> Vec<String> {
    let mut let_byte_code = Vec::new();
    // Dump let keyword
    token_tail.next();
    // Get identifier kind, type and index
    let identifier_token = token_tail.next().unwrap();
    let Var {kind, typ, idx} = lookup(identifier_token, class_table, subroutine_table);

    // check if array-indexing occurs
    if token_tail.peek().unwrap().value == "[" {
        // Dump opening square-bracket
        token_tail.next();
        // Add expression inside square-brackets
        let_byte_code.extend(compile_expression(token_tail, class_table, subroutine_table));
        // Dump closing square-bracket
        token_tail.next();
    }

    // Dump equal sign
    token_tail.next();
    // Handle Expression on right sight of assignment
    let_byte_code.extend(compile_expression(token_tail, class_table, subroutine_table));
    // Assign expression to identifier on left side
    let_byte_code.push(write_pop(&kind, &typ, idx));

    // Dump semicolon and return result 
    let semicolon = token_tail.next().unwrap();
    if semicolon.value != ";" {
        panic!("End of let-statement expects ; but got {}", semicolon.value);
    }
    let_byte_code
}

// Compile DO
fn compile_do(token_tail: &mut TokenStream, class_table: &ClassTable, subroutine_table: &SubroutineTable) -> Vec<String> {
    
    let mut do_byte_code = Vec::new();

    // Add do, [className,.,] subroutine-call (which is some name and a expression-list)
    let _do_keyword_token = token_tail.next().unwrap();

    do_byte_code.extend(compile_subroutine_call(token_tail, class_table, subroutine_table));

    // Next Token must be Semicolon -> dump it
    let semicolon = token_tail.next().unwrap();
    if  semicolon.value != ";" {
        panic!("Do-Statement must be followed by ; but got '{}'", semicolon.value);
    }
    do_byte_code
}
fn compile_subroutine_call(token_tail: &mut TokenStream, class_table: &ClassTable, subroutine_table: &SubroutineTable) -> Vec<String> {
    let mut subroutine_call_byte_code = Vec::new();
    let mut function_name = token_tail.next().unwrap().value.to_string();

    if token_tail.peek().unwrap().value == "." {
        println!("DOT appeared, so second-half of CALL is created");
        // Add dot and second-function-part to function_name
        let dot_token = token_tail.next().unwrap();
        function_name.push_str(&dot_token.value);
        let second_identifier_token = token_tail.next().unwrap();
        function_name.push_str(&second_identifier_token.value);
        // Dump opening paranthese
        token_tail.next();
        let (args, expression_list_byte_code) = compile_expression_list(token_tail, class_table, subroutine_table);
        subroutine_call_byte_code.extend(expression_list_byte_code);
        let function_call = format!("call {} {}", function_name, args);
        subroutine_call_byte_code.push(function_call);
        // Dump closing paranthese
        println!("Dumped {} in compile_subroutine.", token_tail.next().unwrap().value);
    } else {
        // Dump opening paranthese
        token_tail.next();
        let (args, expression_list_byte_code) = compile_expression_list(token_tail, class_table, subroutine_table);
        subroutine_call_byte_code.extend(expression_list_byte_code);
        let function_call = format!("call {} {}", function_name, args);
        subroutine_call_byte_code.push(function_call);
        // Dump closing paranthese
        println!("Dumped {} in compile_subroutine.", token_tail.next().unwrap().value);
    }
    // Do NOT Dump semicolon
    subroutine_call_byte_code
}


// Compile RETURN
fn compile_return(token_tail: &mut TokenStream, class_table: &ClassTable, subroutine_table: &SubroutineTable) -> Vec<String> {
    
    let mut return_byte_code = Vec::new();
    // add return-keyword
    let maybe_return_token = token_tail.next();
    // add expressions (if present)
    return_byte_code.extend(compile_expression(token_tail, class_table, subroutine_table));

    return_byte_code.push(write_return("NOTYPE"));
    if let Some(return_token) = maybe_return_token {
        match return_token.value.as_ref() {
            "return" => return_byte_code.push(String::from("return")),
            other => panic!("Expected return-keyword but got '{}'", other),
        }
    } else {
        panic!("Expected return-keyword but Token was NONE!");
    }
    
    // Ignore semicolon after return-statement
    let semicolon = token_tail.next().unwrap();
    if semicolon.value != ";" {
        panic!("Expected ; at the end of return but got '{:?}", semicolon);
    }
    return_byte_code
}

// Compile ELSE
// fn compile_else(token_tail: &mut TokenStream, class_table: &ClassTable, subroutine_table: &SubroutineTable) -> String {
//     let mut result_else_xml = String::new();
//     // add else-keyword
//     result_else_xml.push_str(&next_as_xml(token_tail));
//     // add opening curly
//     result_else_xml.push_str(&next_as_xml(token_tail));
//     // add else body
//     result_else_xml.push_str(&compile_statement_body(token_tail, class_table, subroutine_table));
    
//     result_else_xml
// }


// Compile EXPRESSION
fn compile_expression(token_tail: &mut TokenStream, class_table: &ClassTable, subroutine_table: &SubroutineTable) -> Vec<String> {
    if token_tail.peek() == None {
        panic!("compile_expression received a TokenStream with no next value. ")
    }
    let mut expression_byte_code = Vec::new();

    // If no term, just return empty Vec
    let next_token = token_tail.peek().unwrap().value.to_string();
    if [")", ",", ";"].contains(&next_token.as_ref())  {
        println!("BROKE IN EXPRESSION with {}", next_token);
        return expression_byte_code;
    }

    // add term
    expression_byte_code.extend(compile_term(token_tail, class_table, subroutine_table));
    
    expression_byte_code
}

fn compile_expression_list(token_tail: &mut TokenStream, class_table: &ClassTable, subroutine_table: &SubroutineTable) -> (u32, Vec<String>) {
    println!("Expression_List got called");
    let mut expression_list_byte_code = Vec::new();
    let mut var_count = 0;
    loop {
        expression_list_byte_code.extend(compile_expression(token_tail, class_table, subroutine_table));
        var_count += 1;
        let next_token = token_tail.peek().unwrap();
        if next_token.value != "," {
            break;
        }
        // Dump Comma
        token_tail.next();
    }
    (var_count, expression_list_byte_code)
}

// Compile term
fn compile_term(token_tail: &mut TokenStream, class_table: &ClassTable, subroutine_table: &SubroutineTable) -> Vec<String> {
    let mut term_byte_code = Vec::new();
    loop {
        // add subunits of term if present
        let token = token_tail.peek();
        // Handle boolean-values
        if token.unwrap().token_type == TokenType::Keyword {
            let boolean = token_tail.next().unwrap();
            match boolean.value.as_ref() {
                "true" => {
                    term_byte_code.push("push constant 1".to_string());
                    term_byte_code.push("neg".to_string());
                },
                "false" => {
                    term_byte_code.push("push constant 0".to_string());
                },
                _ => panic!("Expected identifier true or false but {} was passed", boolean.value),
            }
            break;
        }
        if let Some(t) = token {
            match t.value.as_ref() {
                // If Term ends -> return
                ")" => {
                    println!("Paranthese break happened in compile_term {}", t.value);
                    break
                }
                ";" => {
                    // Do NOT Dump Semicolon
                    println!("Semicolon break happened in compile_term {}", t.value);
                    break
                },
                // Handle Term in parantheses
                "(" => {
                    // Dump open paranthese
                    token_tail.next();
                    // Add Expression inside parantheses
                    term_byte_code.extend(compile_expression(token_tail, class_table, subroutine_table));
                    // Dump closing paranthese
                    token_tail.next();
                },
                // Handle unary-operators
                "-" | "~" => {
                    let unaray_op = token_tail.next().unwrap();
                    term_byte_code.extend(compile_term(token_tail, class_table, subroutine_table));
                    term_byte_code.push(write_unary_op(unaray_op));
                },
                "[" => (), // TODO: Handle indexing
                // Must be single Term, so check for Operators, followed by more term(s)
                _ => {
                    // Save current State of tokens withouth moving cursor
                    let mut tokens_cloned = token_tail.clone();
                    // Get next token
                    let term = token_tail.next().unwrap();
                    // Peek one token further ahead
                    let next_token = token_tail.peek().unwrap();
                    if next_token.value == "." || next_token.value == "(" {
                        // Pass the Token-Clone, so that first part of call is not picked off already
                        term_byte_code.extend(compile_subroutine_call(&mut tokens_cloned, class_table, subroutine_table));
                        *token_tail = tokens_cloned;
                        break;
                    }
                    let Var {kind, typ, idx} = lookup(&term, &class_table, &subroutine_table);
                    if OPERATORS.contains(&next_token.value.as_ref()) {
                        term_byte_code.push(write_push(&kind, &typ, idx));
                        let op = token_tail.next().unwrap();
                        // Add term
                        term_byte_code.extend(compile_term(token_tail, class_table, subroutine_table));
                        // Add op as postfix
                        term_byte_code.push(write_op(op));
                    } else {
                        term_byte_code.push(write_push(&kind, &typ, idx));
                        break;
                    }
                },
            }
        } else {
            // If this is reached, Token mus be None -> something went wrong
            panic!("No Token is '{:?}', but compile_term has been called.", token);
        }
    }
    term_byte_code
}



// #######################
// #######  TESTS  #######
// #######################
//
// allover-compilation-TESTS
#[cfg(test)]
mod tests {
    use super::*;

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
        let mock_result_xml = vec![String::from("TODO")];
        assert_eq!(analyze_tokens(mock_tokens), mock_result_xml);
    }
    
    // class-body-builder-TESTS
    #[test]
    fn class_body_is_correct() {
        let dummy_tokens = tokenize("field int x; }");
        let result_xml = vec![String::from("TODO")];
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
    
    
    // #[test]
    // fn field_var_compiles() {
    //     let dummy_field = "field int y;";
    //     let dummy_field_tokens = tokenize(dummy_field);
    //     let dummy_result = String::from("\
    //         <classVarDec>\
    //             <keyword> field </keyword>\
    //             <keyword> int </keyword>\
    //             <identifier> y </identifier>\
    //             <symbol> ; </symbol>\
    //         </classVarDec>");
    //     assert_eq!(compile_class_vars(&mut dummy_field_tokens.iter().peekable(), &mut ClassTable::default()), dummy_result);
    // }
    // #[test]
    // fn static_var_compiles() {
    //     let dummy_field = "static int num;";
    //     let dummy_field_tokens = tokenize(dummy_field);
    //     let dummy_result = vec![String::from("TODO")];
    //     assert_eq!(compile_class_vars(&mut dummy_field_tokens.iter().peekable(), &mut ClassTable::default()), dummy_result);
    // }
    
    // ### Subroutine-TESTS ###
    #[test]
    fn subroutine_compiled() {
        let dummy_subroutine = "\
            function char myfunc(int age, boolean isCool) {\
                var char letter;\
                var int max, min;\
            }";
        let mock_body_tokens = tokenize(dummy_subroutine);
        // Update subroutine-table
        let TODO = compile_subroutine(&mut mock_body_tokens.iter().peekable(), "_class_name", &ClassTable::default());
        assert_eq!(vec!["TODO".to_string()], TODO);
    }
    
    // Subroutine-Body-TESTS
    #[test]
    fn subroutine_body_updates_local_var_table() {
        let mock_body = "\
            var int num, count;\
            var boolean isOpen;\
        }";
        let mut dummy_subroutine_table = SubroutineTable::default();
        let mock_body_tokens = tokenize(mock_body);
        // Update subroutine-table
        compile_subroutine_body(&mut mock_body_tokens.iter().peekable(), &ClassTable::default(), &mut dummy_subroutine_table);
        assert_eq!(Some(Var {kind: "local".to_string(), typ: "int".to_string(), idx: 0}), dummy_subroutine_table.get("num"));
        assert_eq!(Some(Var {kind: "local".to_string(), typ: "count".to_string(), idx: 1}), dummy_subroutine_table.get("isOpen"));
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
        let dummy_statement_body_xml = vec![String::from("TODO")];
        assert_eq!(compile_statement_body(&mut dummy_statement_body_tokens.iter().peekable(), &ClassTable::default(), &SubroutineTable::default()), dummy_statement_body_xml);
    }
    
    // If Statement-TEST
    // #[test]
    // fn if_else_while_statements_compile() {
    //     let dummy_if_while = "\
    //         if (true) {\
    //             while (false) {\
    //             }\
    //         } else {\
    //         }\
    //     }"; // Extra curly indicating that next token is end of sourrounding subroutine
    //     let dummy_if_while_tokens = tokenize(dummy_if_while);
    //     let dummy_if_while_xml = vec![String::from("TODO")];
    //     assert_eq!(compile_statement(&mut dummy_if_while_tokens.iter().peekable(), &ClassTable::default(), &SubroutineTable::default()), dummy_if_while_xml);
    // }
    // #[test]
    // fn if_several_terms_compiles() {
    //     let dummy_if_while = "if (((y + size) < 254) & ((x + size) < 510)) {} }"; // Extra curly indicating that next token is end of sourrounding subroutine
    //     let dummy_if_while_tokens = tokenize(dummy_if_while);
    //     let dummy_if_while_xml = vec![String::from("TODO")];
    //     assert_eq!(compile_statement(&mut dummy_if_while_tokens.iter().peekable(), &ClassTable::default(), &SubroutineTable::default()), dummy_if_while_xml);
    // }
                
    // Conditionals Test (sub-piece of if and while)
    // #[test]
    // fn conditionals_compile() {
    //     let dummy_condition = "if (true) {}\
    //         }"; // sourrounding statement or subroutine end-curly
    //     let condition_tokens = tokenize(dummy_condition);
    //     let condition_xml = vec![String::from("TODO")];
    //     assert_eq!(compile_conditional_statement(&mut condition_tokens.iter().peekable(), &ClassTable::default(), &SubroutineTable::default()), condition_xml);
    // }
    
    // Do - Test
    #[test]
    fn do_subroutine_compiles() {
        let dummy_do = "do rockit(x);";
        let dummy_do_tokens = tokenize(dummy_do);
        let dummy_do_xml = vec![String::from("TODO")];
            assert_eq!(compile_do(&mut dummy_do_tokens.iter().peekable(), &ClassTable::default(), &SubroutineTable::default()), dummy_do_xml);
    }
    #[test]
    fn do_multiple_expressions_compiles() {
        let dummy_do = "do rockit(x, y, z);";
        let dummy_do_tokens = tokenize(dummy_do);
        let dummy_do_xml = vec![String::from("TODO")];
            assert_eq!(compile_do(&mut dummy_do_tokens.iter().peekable(), &ClassTable::default(), &SubroutineTable::default()), dummy_do_xml);
    }
        
    #[test]
    fn do_class_dot_subroutine_compiles() {
        let dummy_do = "do Great.rockit(x);";
        let dummy_do_tokens = tokenize(dummy_do);
        let dummy_do_xml = vec![String::from("TODO")];
        assert_eq!(compile_do(&mut dummy_do_tokens.iter().peekable(), &ClassTable::default(), &SubroutineTable::default()), dummy_do_xml);
    }
    
    //Return - TESTS
    #[test]
    fn return_without_expression_compiles() {
        let dummy_return_tokens = tokenize("return;");
        let dummy_return = vec!["return".to_string()];
        assert_eq!(compile_return(&mut dummy_return_tokens.iter().peekable(), &ClassTable::default(), &SubroutineTable::default()), dummy_return);
    }
    #[test]
    fn return_with_expression_compiles() {
        let dummy_return_tokens = tokenize(r#"return "cool";"#);
        let dummy_return = vec![String::from("TODO")];
        assert_eq!(compile_return(&mut dummy_return_tokens.iter().peekable(), &ClassTable::default(), &SubroutineTable::default()), dummy_return);
    }
    
    // ### Expressions TESTS ###
    #[test]
    fn expressionless_compiles() {
        let dummy_exp_tokens = tokenize("x;");
        let dummy_var = Var::new("local", "int", 0);
        let mut dummy_subroutine_table = SubroutineTable::default();
        dummy_subroutine_table.add("local", dummy_var);
        let dummy_exp = vec![String::from("push local 0")];
        assert_eq!(compile_expression(&mut dummy_exp_tokens.iter().peekable(), &mut ClassTable::default(), &mut dummy_subroutine_table), dummy_exp);
    }
    
    // term - TESTS (part of expression)
    #[test]
    fn term_compiles() {
        let dummy_term_tokens = tokenize("1 / 2;");
        let dummy_term = vec!["push constant 1".to_string(), "push constant 2".to_string(), "call Math.devide 2".to_string()];
        assert_eq!(compile_term(&mut dummy_term_tokens.iter().peekable(), &mut ClassTable::default(), &mut SubroutineTable::default()), dummy_term);
    }
}
