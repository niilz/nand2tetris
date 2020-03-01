use super::token::{ Token, TokenType, TokenStream };
use super::tokenizer::{ tokenize };
use super::tables::{ Var, ClassTable, SubroutineTable };

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
    class_byte_code.push(String::from("// Class-Body START"));
    let body = build_class_body(&mut token_stream, &class_name.value);
    class_byte_code.extend(body);
    class_byte_code.push(String::from("// Class-Body END"));
    
    // after body has finished, check for closing curly
    let closing_curly = &token_stream.next().unwrap().value;
    if closing_curly != "}" {
        panic!("Class-Files should end with closing curly bracket, but token was '{}'", closing_curly);
    };

    match token_stream.next() {
        None => {
            println!("EOF, Parsing has finished successfully");
            class_byte_code
        },
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
            format!("// Class-Var-Dec Fields: #{}, Statics: #{}",
                class_table.get_next_idx("field"), class_table.get_next_idx("static")));
    }

    // Add subroutines
    class_body_byte_code.push(String::from("// Subroutine START"));
    while token_tail.peek().unwrap().value != "}" {
        class_body_byte_code.extend(compile_subroutine(&mut token_tail, class_name, &class_table));
    }
    class_body_byte_code.push(String::from("// Subroutine END"));
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
fn lookup(var_name: &str, class_table: &ClassTable, subroutine_table: &SubroutineTable) -> Var {
    match subroutine_table.get(var_name) {
        Some(var) => var,
        None => match class_table.get(var_name) {
            Some(var) => var,
            None => panic!("Variable '{:?}' has not been declared.", var_name),
        }
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
    subroutine_byte_code.push(format!("function {}.{}", class_name, routine_name.value));
    // result_subroutine_xml.push_str(&format!("<SUBROUTINE_DEC>{}</SUBROUTINE_DEC>", routine_name.value));
    
    if routine_keyword.value == "method" {
        // Create this-arg
        let this = Var::new("arg", class_name, 0);
        // Add Var to Subrroutine-Table
        subroutine_table.add("this", this);
    }
    // Add parameters to Subroutine-Table (no code-creation)
    let _param_list = compile_paramlist(token_tail, &mut subroutine_table);
    subroutine_byte_code.push(format!("// Parameterlist handled Args: #{}", subroutine_table.get_next_idx("arg")));

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
        subroutine_body_byte_code.push(format!("// Local-Var-Dec Locals: #{}", subroutine_table.get_next_idx("local")));
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

// ### Statements TESTS ###

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
            //"let" => statement_byte_code.extend(compile_let(token_tail, class_table, subroutine_table)),
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
// fn compile_let(token_tail: &mut TokenStream, class_table: &ClassTable, subroutine_table: &SubroutineTable) -> String {
//     let mut result_let_xml = String::from("<letStatement>");
//     // get let keyword
//     result_let_xml.push_str(&next_as_xml(token_tail));
//     // add identifier
//     let identifier_token = token_tail.next().unwrap();
//     result_let_xml.push_str(&identifier_token.to_xml());
//     let identifier = lookup(&identifier_token.value, class_table, subroutine_table);
//     result_let_xml.push_str(&identifier.to_xml());
//     result_let_xml.push_str(&format!("<USE>{}</USE>", identifier_token.value));

//     // check if array-indexing occurs
//     if token_tail.peek().unwrap().value == "[" {
//         // add opening square-bracket
//         result_let_xml.push_str(&next_as_xml(token_tail));

//         // add expression inside square-brackets
//         result_let_xml.push_str(&compile_expression(token_tail, class_table, subroutine_table));

//         // add closing square-bracket
//         result_let_xml.push_str(&next_as_xml(token_tail));
//     }

//     // add equal sign
//     result_let_xml.push_str(&next_as_xml(token_tail));

//     // Add Expression on right sight of assignment
//     result_let_xml.push_str(&compile_expression(token_tail, class_table, subroutine_table));

//     // add semicolon and return result 
//     result_let_xml + &next_as_xml(token_tail) + "</letStatement>"
// }

// Compile DO
fn compile_do(token_tail: &mut TokenStream, class_table: &ClassTable, subroutine_table: &SubroutineTable) -> Vec<String> {
    
    let mut do_byte_code = Vec::new();

    // Add do, [className,.,] subroutine as xml
    let _do_keyword_token = token_tail.next().unwrap();
    let mut function_name = token_tail.next().unwrap().value.to_string();

    if token_tail.peek().unwrap().value == "." {
        // Add dot and second-function-part to function_name
        let dot_token = token_tail.next().unwrap();
        function_name.push_str(&dot_token.value);
        let second_identifier_token = token_tail.next().unwrap();
        function_name.push_str(&second_identifier_token.value);
    }
    // Add function name/label TODO: num of arguments

    // Ignore opening expression-list paranthese
    token_tail.next();
    // add expression-list
    do_byte_code.push(String::from("// Do-Args START"));
    do_byte_code.extend(compile_expression(token_tail, class_table, subroutine_table));
    do_byte_code.push(String::from("// Do-Args END"));
    
    // Ignore closing expression-list paranthese
    token_tail.next();
    // Ignore closing expression-list  semicolon
    token_tail.next();

    do_byte_code
}


// Compile RETURN
fn compile_return(token_tail: &mut TokenStream, class_table: &ClassTable, subroutine_table: &SubroutineTable) -> Vec<String> {
    
    let mut return_byte_code = Vec::new();
    // add return-keyword
    return_byte_code.push(format!("TODO: Write return {:?}", token_tail.next().unwrap().value));
    // add expressions (if present)
    return_byte_code.extend(compile_expression(token_tail, class_table, subroutine_table));
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
    if [")", ";", "]"].contains(&next_token.as_ref())  {
        return expression_byte_code;
    }

    // add term
    expression_byte_code.push(String::from("// Term START"));
    expression_byte_code.extend(compile_term(token_tail, class_table, subroutine_table));
    expression_byte_code.push(String::from("// Term END"));
    
    // add more expressions if there are any
    if token_tail.peek().unwrap().value == "," {
        // Ignore Comma
        token_tail.next();
        expression_byte_code.extend(compile_expression(token_tail, class_table, subroutine_table));
        // maybe: return expression_byte_code;
    }
    expression_byte_code
}

// Compile term
fn compile_term(token_tail: &mut TokenStream, class_table: &ClassTable, subroutine_table: &SubroutineTable) -> Vec<String> {
    let mut term_byte_code = Vec::new();
    // add nested unary-op-exprssion if present
    // TODO: UNARY-OP
    // if UNARY_OP.contains(&token_tail.peek().unwrap().value.as_ref()) {
    //     // start nested expression
    //     result_term_xml.push_str("<term>");
    //     // add unary-operator
    //     result_term_xml.push_str(&next_as_xml(token_tail));
    //     // add the nested term
    //     result_term_xml.push_str(&compile_term(token_tail, "", class_table, subroutine_table));
    //     // finish nested expression
    //     result_term_xml.push_str("</term>");
    // }

    let next_token = token_tail.peek().unwrap().value.to_string();
    // add array indexing or exressions if present
    if ["(", "["].contains(&next_token.as_ref()) {
        // result_term_xml.push_str(if next_token == "(" { "<term>" } else { "" });
        
        // Ignore opening bracket/parantese
        token_tail.next();
        // add expression
        term_byte_code.extend(compile_expression(token_tail, class_table, subroutine_table));
        // Ignore closing bracket/parantese
        token_tail.next();
        // result_term_xml.push_str(if next_token == "(" { "</term>" } else { "" });
        // add more terms if there are any
        if OPERATORS.contains(&token_tail.peek().unwrap().value.as_ref()) {
            // first add the term
            term_byte_code.extend(compile_term(token_tail, class_table, subroutine_table));
            // then add the operator
            term_byte_code.push(token_tail.next().unwrap().value.to_string());
        }
    }

    // if term is finished return it
    if [")", ";", "]", ","].contains(&token_tail.peek().unwrap().value.as_ref())  {
        // ignore closing thing
        token_tail.next();
        return term_byte_code;
    }
    
    // start adding Expression term(s)
    term_byte_code.push(String::from("// Term inside <term> START"));
    
    let term_token = token_tail.next().unwrap();
    
    // add subunits of term if present
    let next_token = token_tail.peek().unwrap().value.to_string();
    // Write call if present
    if &next_token == "." {
        // Call statement base
        let mut call_code = format!("call {}", term_token.value);
        let dot = token_tail.next().unwrap();
        call_code.push_str(&dot.value);
        let second_call_part_token = token_tail.next().unwrap();
        call_code.push_str(&second_call_part_token.value);
        term_byte_code.push(call_code);
        // TODO: arguments count
        
        // Ignore opening ExpressionList paranthese
        token_tail.next();
        term_byte_code.push(String::from("expressionList of call START"));
        term_byte_code.extend(compile_expression(token_tail, class_table, subroutine_table));
        term_byte_code.push(String::from("expressionList of call START"));
        // Ignore closing Expression-List paranthese
        token_tail.next();
    } else if term_token.token_type == TokenType::Identifier {
        // First token must be Identifier, look up and add Data
        let var = lookup(&term_token.value, class_table, subroutine_table);
        // TODO: propper push commands in VM
        term_byte_code.push(format!("Var Name: {} and DATA: {:?}", term_token.value, var));
    } else {
        // Add single term (like number)
        term_byte_code.push(format!("Should be pushed: {}", term_token.value));
    }

    // TODO: INDEXING
    // if next_token == "[" {
    //     // add indexing part of term if present
    //     result_term_xml.push_str(&compile_term(token_tail, "", class_table, subroutine_table));
    // }

    // end adding Expression term
    term_byte_code.push(String::from("// Term inside <term> End"));
    
    // add op if available
    let mut op = None;
    if OPERATORS.contains(&token_tail.peek().unwrap().value.as_ref()) {
        // add operator
        op = Some(token_tail.next().unwrap().value.to_string());
    }
    term_byte_code.extend(compile_term(token_tail, class_table, subroutine_table));
    if op != None {
        term_byte_code.push(format!("OPERATOR: {}", op.unwrap()));
    }
    term_byte_code
}



// #######################
// #######  TESTS  #######
// #######################
//
// allover-integration-TESTS
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
        let dummy_subroutine_tokens = tokenize(dummy_subroutine);
        let dummy_subroutine_xml = vec![String::from("TODO")];
        assert_eq!(compile_subroutine(&mut dummy_subroutine_tokens.iter().peekable(), "_class_name", &ClassTable::default()), dummy_subroutine_xml);
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
        let mock_body_xml = vec![String::from("TODO")];
        assert_eq!(compile_subroutine_body(&mut mock_body_tokens.iter().peekable(), &ClassTable::default(), &mut SubroutineTable::default(), ), mock_body_xml);
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
    
    // Let Statement-TEST
    // #[test]
    // fn let_wihtout_expression_compiles() {
    //     let dummy_let_tokens = tokenize("let myVar = 50;");
    //     let dummy_let_xml = "\
    //         <letStatement>\
    //             <keyword> let </keyword>\
    //             <identifier> myVar </identifier>\
    //             <symbol> = </symbol>\
    //             <expression>\
    //                 <term>\
    //                     <integerConstant> 50 </integerConstant>\
    //                 </term>\
    //             </expression>\
    //             <symbol> ; </symbol>\
    //         </letStatement>";
    //     assert_eq!(compile_let(&mut dummy_let_tokens.iter().peekable(), &ClassTable::default(), &SubroutineTable::default()), dummy_let_xml);
    // }
    // #[test]
    // fn let_with_or_compiles() {
    //     let dummy_let_tokens = tokenize("let myVar = 50 | 60;");
    //     let dummy_let_xml = "\
    //         <letStatement>\
    //             <keyword> let </keyword>\
    //             <identifier> myVar </identifier>\
    //             <symbol> = </symbol>\
    //             <expression>\
    //                 <term>\
    //                     <integerConstant> 50 </integerConstant>\
    //                 </term>\
    //                 <symbol> | </symbol>\
    //                 <term>\
    //                     <integerConstant> 60 </integerConstant>\
    //                 </term>\
    //             </expression>\
    //             <symbol> ; </symbol>\
    //         </letStatement>";
    //     assert_eq!(compile_let(&mut dummy_let_tokens.iter().peekable(), &ClassTable::default(), &SubroutineTable::default()), dummy_let_xml);
    // }
    
    // #[test]
    // fn let_with_array_idx_compiles() {
    //     let dummy_let_tokens = tokenize("let myVar[i] = 50;");
    //     let dummy_let_xml = "\
    //         <letStatement>\
    //             <keyword> let </keyword>\
    //             <identifier> myVar </identifier>\
    //             <symbol> [ </symbol>\
    //               <expression>\
    //                 <term>\
    //                   <identifier> i </identifier>\
    //                 </term>\
    //               </expression>\
    //               <symbol> ] </symbol>\
    //             <symbol> = </symbol>\
    //             <expression>\
    //                 <term>\
    //                     <integerConstant> 50 </integerConstant>\
    //                 </term>\
    //             </expression>\
    //             <symbol> ; </symbol>\
    //         </letStatement>";
    //     assert_eq!(compile_let(&mut dummy_let_tokens.iter().peekable(), &ClassTable::default(), &SubroutineTable::default()), dummy_let_xml);
    // }
    #[test]
    // fn let_subroutine_call_compiles() {
    //     let dummy_let_tokens = tokenize("let subR = myFunc.call();");
    //     let dummy_let_xml = "\
    //         <letStatement>\
    //             <keyword> let </keyword>\
    //             <identifier> subR </identifier>\
    //             <symbol> = </symbol>\
    //             <expression>\
    //                 <term>\
    //                     <identifier> myFunc </identifier>\
    //                     <symbol> . </symbol>\
    //                     <identifier> call </identifier>\
    //                     <symbol> ( </symbol>\
    //                     <expressionList>\
    //                     </expressionList>\
    //                     <symbol> ) </symbol>\
    //                 </term>\
    //             </expression>\
    //             <symbol> ; </symbol>\
    //         </letStatement>";
    //     assert_eq!(compile_let(&mut dummy_let_tokens.iter().peekable(), &ClassTable::default(), &SubroutineTable::default()), dummy_let_xml);
    // }
    // #[test]
    // fn let_array_idx_compiles() {
    //     let dummy_let_tokens = tokenize("let a[1]= blup;");
    //     let dummy_let_xml = "\
    //         <letStatement>\
    //             <keyword> let </keyword>\
    //             <identifier> a </identifier>\
    //             <symbol> [ </symbol>\
    //               <expression>\
    //                 <term>\
    //                   <integerConstant> 1 </integerConstant>\
    //                 </term>\
    //               </expression>\
    //               <symbol> ] </symbol>\
    //             <symbol> = </symbol>\
    //             <expression>\
    //                 <term>\
    //                     <identifier> blup </identifier>\
    //                 </term>\
    //             </expression>\
    //             <symbol> ; </symbol>\
    //         </letStatement>";
    //     assert_eq!(compile_let(&mut dummy_let_tokens.iter().peekable(), &ClassTable::default(), &SubroutineTable::default()), dummy_let_xml);
    // }
    // #[test]
    // fn let_with_parantheses_compiles() {
    //     let dummy_let_tokens = tokenize("let a = i * (-3);");
    //     let dummy_let_xml = "\
    //         <letStatement>\
    //             <keyword> let </keyword>\
    //             <identifier> a </identifier>\
    //             <symbol> = </symbol>\
    //             <expression>\
    //                 <term>\
    //                   <identifier> i </identifier>\
    //                 </term>\
    //                 <symbol> * </symbol>\
    //                 <term>\
    //                   <symbol> ( </symbol>\
    //                   <expression>\
    //                     <term>\
    //                       <symbol> - </symbol>\
    //                       <term>\
    //                         <integerConstant> 3 </integerConstant>\
    //                       </term>\
    //                     </term>\
    //                   </expression>\
    //                   <symbol> ) </symbol>\
    //                 </term>\
    //               </expression>\
    //             <symbol> ; </symbol>\
    //         </letStatement>";
    //     assert_eq!(compile_let(&mut dummy_let_tokens.iter().peekable(), &ClassTable::default(), &SubroutineTable::default()), dummy_let_xml);
    // }
    // #[test]
    // fn let_with_square_term_right_compiles() {
    //     let dummy_let_tokens = tokenize("let sum = sum + a[i];");
    //     let dummy_let_xml = "\
    //     <letStatement>\
    //         <keyword> let </keyword>\
    //         <identifier> sum </identifier>\
    //         <symbol> = </symbol>\
    //         <expression>\
    //         <term>\
    //             <identifier> sum </identifier>\
    //         </term>\
    //         <symbol> + </symbol>\
    //         <term>\
    //             <identifier> a </identifier>\
    //             <symbol> [ </symbol>\
    //             <expression>\
    //             <term>\
    //                 <identifier> i </identifier>\
    //             </term>\
    //             </expression>\
    //             <symbol> ] </symbol>\
    //         </term>\
    //         </expression>\
    //         <symbol> ; </symbol>\
    //     </letStatement>";
    //     assert_eq!(compile_let(&mut dummy_let_tokens.iter().peekable(), &ClassTable::default(), &SubroutineTable::default()), dummy_let_xml);
    // }
    
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
    
    // Return - TESTS
    // #[test]
    // fn return_without_expression_compiles() {
    //     let dummy_return_tokens = tokenize("return;");
    //     let dummy_return_xml = "\
    //         <returnStatement>\
    //             <keyword> return </keyword>\
    //             <symbol> ; </symbol>\
    //         </returnStatement>";
    //     assert_eq!(compile_return(&mut dummy_return_tokens.iter().peekable(), &ClassTable::default(), &SubroutineTable::default()), dummy_return_xml);
    // }
    // #[test]
    // fn return_with_expression_compiles() {
    //     let dummy_return_tokens = tokenize(r#"return "cool";"#);
    //     let dummy_return_xml = "\
    //         <returnStatement>\
    //             <keyword> return </keyword>\
    //             <expression>\
    //                 <term>\
    //                     <stringConstant> cool </stringConstant>\
    //                 </term>\
    //             </expression>\
    //             <symbol> ; </symbol>\
    //         </returnStatement>";
    //     assert_eq!(compile_return(&mut dummy_return_tokens.iter().peekable(), &ClassTable::default(), &SubroutineTable::default()), dummy_return_xml);
    // }
    
    // ### Expressions TESTS ###
    #[test]
    fn expressionless_compiles() {
        let dummy_exp = "x;";
        let dummy_exp_tokens = tokenize(dummy_exp);
        let dummy_exp_xml = vec![String::from("TODO")];
        assert_eq!(compile_expression(&mut dummy_exp_tokens.iter().peekable(), &mut ClassTable::default(), &mut SubroutineTable::default()), dummy_exp_xml);
    }
    
    // term - TESTS (part of expression)
    #[test]
    fn term_compiles() {
        let dummy_term_tokens = tokenize("i / 2;");
        let dummy_term_xml = vec![String::from("TODO")];
        assert_eq!(compile_term(&mut dummy_term_tokens.iter().peekable(), &mut Vec::new(), &mut ClassTable::default(), &mut SubroutineTable::default()), dummy_term_xml);
    }
}
