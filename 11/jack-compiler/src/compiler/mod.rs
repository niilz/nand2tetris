pub mod tables;
pub mod code_writer;

use crate::tokenizer::token::{ Token, TokenType, TokenStream };
use crate::tokenizer::{ tokenize };
use tables::{ Var, ClassTable, SubroutineTable };
use code_writer::*;

static OPERATORS: &[&str] = &["+", "-", "*", "/", "&", "|", "<", ">", "=", "~"];
static UNARY_OP: &[&str] = &["-", "~"];

pub struct Compiler<'a> {
    token_tail: TokenStream<'a>,
    class_name: &'a str,
    class_table: ClassTable,
    subroutine_table: SubroutineTable,
    current_subroutine: Subroutine<'a>,
}
#[derive(Default)]
pub struct Subroutine<'a> {
    name: &'a str,
    return_type: ReturnType<'a>,
}
#[derive(Debug)]
enum ReturnType<'a> {
    Void,
    Int,
    Char,
    Boolean,
    Class(&'a str),
}
impl<'a> Default for ReturnType<'a> {
    fn default() -> Self {
        ReturnType::Void
    }
}

impl<'a> Compiler<'a> {
    pub fn new(tokens: &'a Vec<Token>, class_name: &'a str) -> Self {
        Compiler {
            token_tail: tokens.iter().peekable(),
            class_name: class_name,
            class_table: ClassTable::default(),
            subroutine_table: SubroutineTable::default(),
            current_subroutine: Subroutine::default(),
        }
    }

    fn set_subroutine(&mut self, name: &'a str, return_type_str: &'a str) {
        let return_type = match return_type_str {
            "void" => ReturnType::Void,
            "int" => ReturnType::Int,
            "boolean" => ReturnType::Boolean,
            "char" => ReturnType::Char,
            class => ReturnType::Class(class),
        };
        self.current_subroutine = Subroutine {
            name,
            return_type,
        };
    }
    fn get_subroutine(&self) -> &Subroutine<'a> {
        &self.current_subroutine
    }
    fn get_subroutine_name(&self) -> &'a str {
        self.current_subroutine.name
    }
    fn get_subroutine_type(&self) -> &ReturnType {
        &self.current_subroutine.return_type
    }

    pub fn analyze_tokens(&mut self) -> Vec<String> {
        let class_keyword = self.token_tail.next().unwrap();
        if class_keyword.value != "class" {
            panic!("class files need to start with class decleration");
        }
        
        let class_name = self.token_tail.next().unwrap();
        if class_name.token_type != TokenType::Identifier {
            panic!("classes need a valid Class-Identifier");
        }
        
        let mut class_byte_code = Vec::new();
        class_byte_code.push(format!("// ByteCode for class '{}'\n", class_name.value));
    
        // skip opening curly-brace
        self.token_tail.next();
        
        // parse the body
        let body = self.build_class_body();
        class_byte_code.extend(body);
        
        // after body has finished, check for closing curly
        let closing_curly = &self.token_tail.next().unwrap().value;
        if closing_curly != "}" {
            panic!("Class-Files should end with closing curly bracket, but token was '{}'", closing_curly);
        };
    
        match self.token_tail.next() {
            None => class_byte_code,
            Some(token) => panic!("Expected eof, but got token {:?}", token), 
        }
    }
    // Compilse class-body
    fn build_class_body(&mut self) -> Vec<String> {
        // let mut class_table = ClassTable::default();
        let mut class_body_byte_code = Vec::new();
    
        loop {
            let next_token = self.token_tail.peek();
            if !is_class_var_start(next_token) {
                break;
            }
            // Has no output, but registers the class-vars
            self.compile_class_vars();
            class_body_byte_code.push(
                format!("// Class-Var-Dec Fields: #{}, Statics: #{}\n",
                    self.class_table.get_next_idx("field"), self.class_table.get_next_idx("static")));
        }
    
        // Add subroutines
        while self.token_tail.peek().unwrap().value != "}" {
            class_body_byte_code.extend(self.compile_subroutine());
        }
        class_body_byte_code
    }
    // Compilation Helpers
    // calles itself until a semicolon (;) appears in the TokenStream
    // Registers Class-Vars in ClassVarTable
    fn compile_class_vars(&mut self) {
    
        let var_kind = &self.token_tail.next().unwrap().value;
        let var_typ = &self.token_tail.next().unwrap().value;
        let var_name = &self.token_tail.next().unwrap().value;
    
        // Construct VAR
        let var = Var::new(&var_kind, &var_typ, self.class_table.get_next_idx(&var_kind));
        // Add Var to Class-Table
        self.class_table.add(&var_name, var);
    
        loop {
            if self.token_tail.peek().unwrap().value == ";" {
                break;
            }
            // ignore Comma
            self.token_tail.next();
            let var_token = self.token_tail.next().unwrap();
            // Create Var and add data to xml
            let var = Var::new(&var_kind, &var_typ, self.class_table.get_next_idx(&var_kind));
            // Add to Class-Table
            self.class_table.add(&var_token.value, var);
        }
        // ignore Semicolon (end of decleration)
        let semicolon = &self.token_tail.next().unwrap().value;
        if semicolon != ";" {
            panic!("Semicolon expected as end of ClassVarDec, but '{:?}' was found.", semicolon);
        }
        self.token_tail.next();
    }
    // soubroutine-compiler
    fn compile_subroutine(&mut self) -> Vec<String> {
        // Reset SubroutineTable
        self.subroutine_table = SubroutineTable::default();
        
        let mut subroutine_byte_code = Vec::new();
        // Add suroutine-keyword, type and subroutine name/identifier
        let routine_keyword = self.token_tail.next().unwrap();
        let return_type = self.token_tail.next().unwrap();
        let routine_name = self.token_tail.next().unwrap();
        // Update/set subroutine (name & type)
        self.set_subroutine(&routine_name.value, &return_type.value);
        
        if routine_keyword.value == "method" {
            // Create this-arg
            let this = Var::new("arg", self.class_name, 0);
            // Add Var to Subrroutine-Table
            self.subroutine_table.add("this", this);
        }
        // Add parameters to Subroutine-Table (no code-creation)
        self.compile_paramlist();
        
        // Ignore opening curly-brace
        self.token_tail.next();
        // Start subroutine-body
        // Add code in subroutine-body
        let subroutine_body = self.compile_subroutine_body();
        // Now the local-var-count is known. So first add the function label, then the body-statements
        subroutine_byte_code.push(
            format!("function {}.{} {}\n", self.class_name, self.get_subroutine_name(), self.subroutine_table.get_next_idx("local")));
        subroutine_byte_code.extend(subroutine_body);
        // End subroutine
        let closing_curly = self.token_tail.next().unwrap();
        if closing_curly.value != "}" {
            panic!("Expected closing curly at end of SubroutineBody, but got '{:?}'", closing_curly);
        }
        subroutine_byte_code
    }
    // Compile PARAM (part of subroutine)
    fn compile_paramlist(&mut self) {
    
        loop {
            let token = self.token_tail.next().unwrap();
    
            if token.value  == ")" { break; }
    
            // If opening paranthese -> ignore it
            if ["(", ","].contains(&token.value.as_ref()) { continue; }
    
            // add param type and name
            let typ_token = token;
            let name_token = self.token_tail.next().unwrap();
            // Create arg-var and add it to Subroutine-Table
            let arg = Var::new("arg", &typ_token.value, self.subroutine_table.get_next_idx("arg"));
            // Add arg to Subroutine-Table
            self.subroutine_table.add(&name_token.value, arg);
        }
    }
    // Compile SOUBROUTINE-BODY (part of subroutine)
    fn compile_subroutine_body(&mut self) -> Vec<String> {
        let mut subroutine_body_byte_code = Vec::new();
        // add var-decleration if there are any
        loop {
            let next_token = self.token_tail.peek().unwrap();
            if next_token.value != "var" {
                break;
            }
            // Registers local Vars (no byte code)
            self.compile_var_dec();
        }
    
        // If closing curly appears, subroutine has no statements and can return early
        if self.token_tail.peek().unwrap().value == "}" {
            return subroutine_body_byte_code;
        }
    
        // add statements
        loop {
            let next_token = self.token_tail.peek().unwrap().value.as_str();
            if next_token == "}" || next_token == ";" {
                break;
            }
            subroutine_body_byte_code.extend(self.compile_statement());
        }
        subroutine_body_byte_code
    }
    // Compile VAR-DECLERATION (part of subroutine-body)
    fn compile_var_dec(&mut self) {
        let var_keyword_token = self.token_tail.next().unwrap();
        let var_type_token = self.token_tail.next().unwrap();
        let var_name_token = self.token_tail.next().unwrap();
        
        // Construct first Var and add it to Subroutine-Table
        let var = Var::new("local", &var_type_token.value, self.subroutine_table.get_next_idx("local"));
        self.subroutine_table.add(&var_name_token.value, var);
    
        loop {
            let next_token = self.token_tail.next().unwrap();
            // If no more Local-Var decs -> quit
            if next_token.value == ";" { break; }
            // Otherwise register more Var(s) of same type (ignore the comma)
            if next_token.value == "," {
                let next_var_name = self.token_tail.next().unwrap();
                // construct var behind comma and add it to Subroutine-Table
                let var = Var::new("local", &var_type_token.value, self.subroutine_table.get_next_idx("local"));
                self.subroutine_table.add(&next_var_name.value, var);
            }
        }
    }
    // Compile STATEMENTS
    fn compile_statement(&mut self) -> Vec<String> {
        let mut statement_byte_code = Vec::new();
        
        let mut statement_count = 1;
        loop {
            if self.token_tail.peek() == None {
                panic!("token_tail has no next value in compile_statement. But should either see have } of sourrounding subroutine-body or next statement or else")
            }
            // Check if no more subroutines?
            if self.token_tail.peek().unwrap().value == "}" {
                break;
            }
            let statement_token_value = self.token_tail.peek().unwrap().value.to_string();
            match statement_token_value.as_str() {
                "let" => statement_byte_code.extend(self.compile_let()),
                "if" | "while" => statement_byte_code.extend(self.compile_conditional_statement(statement_count)),
                "do" => statement_byte_code.extend(self.compile_do()),
                "return" => statement_byte_code.extend(self.compile_return()),
                s => panic!("unexpected statement-keyword of: {:?}", s),
            }
            statement_count += 1;
        }
        statement_byte_code
    }
    // Compile Statement body
    fn compile_statement_body(&mut self) -> Vec<String> {
        let mut statement_body_byte_code = Vec::new();
        // If body is not empty, get more statements
        if self.token_tail.peek() == None {
            panic!("no next value available in compile_statement_body. Either } of this statement should be there or more statements");
        }
        if self.token_tail.peek().unwrap().value != "}" {
            statement_body_byte_code.extend(self.compile_statement());
        }
        // Check for closing curly and then ignore it
        let closing_curly = self.token_tail.next().unwrap();
        if closing_curly.value != "}" {
            panic!("End of StatementBody expects closing }} but got '{:?}' ", closing_curly);
        }
        statement_body_byte_code
    }
    // Compile LET
    pub fn compile_let(&mut self) -> Vec<String> {
        let mut let_byte_code = Vec::new();
        // Dump let keyword
        self.token_tail.next();
        // Get identifier kind, type and index
        let identifier_token = self.token_tail.next().unwrap();
        let Var {kind, typ, idx} = lookup(identifier_token, &self.class_table, &self.subroutine_table);
    
        // check if array-indexing occurs
        if self.token_tail.peek().unwrap().value == "[" {
            // Dump opening square-bracket
            self.token_tail.next();
            // Add expression inside square-brackets
            let_byte_code.extend(self.compile_expression());
            // Dump closing square-bracket
            self.token_tail.next();
        }
    
        // Dump equal sign
        let equal_sign = self.token_tail.next().unwrap();
        if equal_sign.value != "=" {
            panic!("Expected = in let assignment but got {}.", equal_sign.value);
        }
        // Handle Expression on right sight of assignment
        let_byte_code.extend(self.compile_expression());
        // Assign expression to identifier on left side
        let_byte_code.push(write_pop(&kind, &typ, idx));
    
        // Dump semicolon and return result 
        let semicolon = self.token_tail.next().unwrap();
        if semicolon.value != ";" {
            panic!("End of let-statement expects ; but got {}", semicolon.value);
        }
        let_byte_code
    }
    // Compile CONDITION statement "if, while"
    fn compile_conditional_statement(&mut self, statement_count: u32) -> Vec<String> {
        let mut conditional_byte_code = Vec::new();
        // Construct start-label (e.g. Myclass.Routine.$1)
        let start_label = format!("{}.{}.${}", self.class_name, self.get_subroutine_name(), statement_count);
        conditional_byte_code.push(format!("label {}", start_label));

        // get keyword
        let statement_token = self.token_tail.next().unwrap();

        // Dump open paranthese
        self.token_tail.next();
        // Add all expression
        conditional_byte_code.extend(self.compile_expression());
        // Dump close paranthese
        self.token_tail.next();

        // Negate expression
        conditional_byte_code.push("neg".to_string());
        // Construct else-label
        let else_label = format!("else.{}", start_label);
        // Jump to else if condition is true after "neg"
        conditional_byte_code.push(format!("if-goto {}", else_label));
        
        // Dump opening curly-brace
        self.token_tail.next();
        // add statement-body (includes closing curly brace)
        conditional_byte_code.extend(self.compile_statement_body());

        // If while -> jump back to start
        if statement_token.value == "while" {
            conditional_byte_code.push(format!("goto {}", start_label));
        }
        // If it got here else must no be performed
        let end_label = format!("end.{}", start_label);
        conditional_byte_code.push(format!("goto {}", end_label));

        // Insert else-Label (to be able to jump to it)
        conditional_byte_code.push(format!("label {}", else_label));
        
        // in case else is following the previous statement add it
        if self.token_tail.peek().unwrap().value == "else" {
            conditional_byte_code.extend(self.compile_else());
        }

        // Insert the end-label (no matter if else is present or not)
        // If no else is present -> else-label is immediately followed by the end-label
        conditional_byte_code.push(format!("label {}", end_label));

        // Return ruslting condition as xml
        conditional_byte_code
    }
    // Compile ELSE
    fn compile_else(&mut self) -> Vec<String> {
        let mut else_byte_code = Vec::new();
        // Dump else-keyword
        self.token_tail.next();
        // Dump opening curly
        self.token_tail.next();
        // add else body
        else_byte_code.extend(self.compile_statement_body());
        
        else_byte_code
    }
    // Compile DO
    fn compile_do(&mut self) -> Vec<String> {
        
        let mut do_byte_code = Vec::new();
    
        // Add do, [className,.,] subroutine-call (which is some name and a expression-list)
        let _do_keyword_token = self.token_tail.next().unwrap();
    
        do_byte_code.extend(self.compile_subroutine_call());
    
        // Next Token must be Semicolon -> dump it
        let semicolon = self.token_tail.next().unwrap();
        if  semicolon.value != ";" {
            panic!("Do-Statement must be followed by ; but got '{}'", semicolon.value);
        }
        do_byte_code.push("pop temp 0".to_string());
        do_byte_code
    }
    // Compile Call to a Subroutine
    fn compile_subroutine_call(&mut self) -> Vec<String> {
        let mut subroutine_call_byte_code = Vec::new();
        let mut function_name = self.token_tail.next().unwrap().value.to_string();
    
        if self.token_tail.peek().unwrap().value == "." {
            println!("DOT appeared, so second-half of CALL is created");
            // Add dot and second-function-part to function_name
            let dot_token = self.token_tail.next().unwrap();
            function_name.push_str(&dot_token.value);
            let second_identifier_token = self.token_tail.next().unwrap();
            function_name.push_str(&second_identifier_token.value);
            // Dump opening paranthese
            self.token_tail.next();
            let (args, expression_list_byte_code) = self.compile_expression_list();
            subroutine_call_byte_code.extend(expression_list_byte_code);
            let function_call = format!("call {} {}", function_name, args);
            subroutine_call_byte_code.push(function_call);
            // Dump closing paranthese
            println!("Dumped {} in compile_subroutine_call.", self.token_tail.next().unwrap().value);
        } else {
            // Dump opening paranthese
            self.token_tail.next();
            let (args, expression_list_byte_code) = self.compile_expression_list();
            subroutine_call_byte_code.extend(expression_list_byte_code);
            let function_call = format!("call {} {}", function_name, args);
            subroutine_call_byte_code.push(function_call);
            // Dump closing paranthese
            println!("Dumped {} in compile_subroutine.", self.token_tail.next().unwrap().value);
        }
        // Dump top value on the stack
        println!("CALLED FUNC {}", self.get_subroutine_name());
        // subroutine_call_byte_code.push("pop temp 0".to_string());
        // Do NOT Dump semicolon
        subroutine_call_byte_code
    }
    // Compile EXPRESSION
    fn compile_expression(&mut self) -> Vec<String> {
        if self.token_tail.peek() == None {
            panic!("compile_expression received a TokenStream with no next value. ")
        }
        let mut expression_byte_code = Vec::new();

        // If no term, just return empty Vec
        let next_token = self.token_tail.peek().unwrap().value.to_string();
        if [")", ",", ";"].contains(&next_token.as_ref())  {
            println!("BROKE IN EXPRESSION with {}", next_token);
            return expression_byte_code;
        }

        // add term
        expression_byte_code.extend(self.compile_term());
        
        expression_byte_code
    }

    fn compile_expression_list(&mut self) -> (u32, Vec<String>) {
        println!("Expression_List got called");
        let mut expression_list_byte_code = Vec::new();
        let mut var_count = 0;
        loop {
            expression_list_byte_code.extend(self.compile_expression());
            var_count += 1;
            let next_token = self.token_tail.peek().unwrap();
            if next_token.value != "," {
                break;
            }
            // Dump Comma
            self.token_tail.next();
        }
        (var_count, expression_list_byte_code)
    }

    // Compile RETURN
    fn compile_return(&mut self) -> Vec<String> {
        
        let mut return_byte_code = Vec::new();
        // add return-keyword
        let maybe_return_token = self.token_tail.next();
        // add expressions (if present)
        return_byte_code.extend(self.compile_expression());

        return_byte_code.push(write_return(&self.get_subroutine()));
        if let Some(return_token) = maybe_return_token {
            match return_token.value.as_ref() {
                "return" => return_byte_code.push(String::from("return\n")),
                other => panic!("Expected return-keyword but got '{}'", other),
            }
        } else {
            panic!("Expected return-keyword but Token was NONE!");
        }
        
        // Ignore semicolon after return-statement
        let semicolon = self.token_tail.next().unwrap();
        if semicolon.value != ";" {
            panic!("Expected ; at the end of return but got '{:?}", semicolon);
        }
        return_byte_code
    }
    // Compile term
    fn compile_term(&mut self) -> Vec<String> {
        let mut term_byte_code = Vec::new();
        // add subunits of term if present
        let token = self.token_tail.peek();
        // Handle boolean-values
        if token.unwrap().token_type == TokenType::Keyword {
            let boolean = self.token_tail.next().unwrap();
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
            return term_byte_code;
        }
        if let Some(t) = token {
            match t.value.as_ref() {
                // If Term ends -> return
                ")" => {}, // !!! is never reached !!!!
                ";" => {
                    // Do NOT Dump Semicolon
                    println!("Semicolon break happened in compile_term {}", t.value);
                },
                // Handle Term in parantheses
                "(" => {
                    // Dump open paranthese
                    self.token_tail.next();
                    // Add Expression inside parantheses
                    term_byte_code.extend(self.compile_expression());
                    // Dump closing paranthese
                    self.token_tail.next();
                    let maybe_op = self.token_tail.peek().unwrap();
                    if OPERATORS.contains(&maybe_op.value.as_ref()) {
                        let op = self.token_tail.next().unwrap();
                        term_byte_code.extend(self.compile_term());
                        term_byte_code.push(write_op(op));
                    }
                },
                // Handle unary-operators
                "-" | "~" => {
                    let unaray_op = self.token_tail.next().unwrap();
                    term_byte_code.extend(self.compile_term());
                    term_byte_code.push(write_unary_op(unaray_op));
                },
                "[" => (), // TODO: Handle indexing
                // Must be single Term, so check for Operators, followed by more term(s)
                _ => {
                    // Save current State of tokens withouth moving cursor
                    let tokens_cloned = self.token_tail.clone();
                    // Get next token
                    let term = self.token_tail.next().unwrap();
                    // Peek one token further ahead
                    let next_token = self.token_tail.peek().unwrap();
                    if next_token.value == "." || next_token.value == "(" {
                        // Pass the Token-Clone, so that first part of call is not picked off already
                        self.token_tail = tokens_cloned;
                        term_byte_code.extend(self.compile_subroutine_call());
                        return term_byte_code;
                    }
                    let Var {kind, typ, idx} = lookup(&term, &self.class_table, &self.subroutine_table);
                    if OPERATORS.contains(&next_token.value.as_ref()) {
                        term_byte_code.push(write_push(&kind, &typ, idx));
                        let op = self.token_tail.next().unwrap();
                        // Add term
                        term_byte_code.extend(self.compile_term());
                        // Add op as postfix
                        term_byte_code.push(write_op(op));
                    } else {
                        term_byte_code.push(write_push(&kind, &typ, idx));
                    }
                },
            }
        } else {
            // If this is reached, Token mus be None -> something went wrong
            panic!("No Token is '{:?}', but compile_term has been called.", token);
        }
        term_byte_code
    }
}

// Check if valid class, right in the beginning
fn is_class_var_start(maybe_token: Option<&&Token>) -> bool {
    let maybe_class_var = &maybe_token.unwrap().value;
    maybe_class_var == "static" || maybe_class_var == "field"
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
        _ => panic!("Lookup for token-type '{:?}' with value '{}' is not implemented", var.token_type, var.value),
    }
}


// #######################
// #######  TESTS  #######
// #######################
//
// allover-compilation-TESTS
// #[cfg(test)]
// mod tests {
//     use super::*;

//     static dummy_code: &'static str = r#"
//         class Great {
//            field int x;
//            static char y;
//            function char myfunc(int age, boolean isCool) {
//                var char letter;
//                var int max, min;
//                if (true) {
//               }
//            }
//            function boolean secondFunc(char a, int 42) {
//                var int size;
//                if (false) {
//                 let x = 1;
//                 return "Hello";
//               }
//            }
//         }"#;
    
    // ### CLASS - TESTS
    // #[test]
    // #[should_panic]
    // fn files_without_class_keyword_panic() {
    //     let mock_token = Token {token_type: TokenType::Keyword, value: String::from("NOCLASS") };
    //     analyze_tokens(vec![mock_token]);
    // }
    // All-in-one - TEST
    // #[test]
    // fn vec_of_tokens_gets_compiled() {
    //     let mock_tokens = tokenize(dummy_code);
    //     let mock_result_xml = vec![String::from("TODO")];
    //     assert_eq!(analyze_tokens(mock_tokens), mock_result_xml);
    // }
    
    // class-body-builder-TESTS
    // #[test]
    // fn class_body_is_correct() {
    //     let dummy_tokens = tokenize("field int x; }");
    //     let result_xml = vec![String::from("TODO")];
    //     assert_eq!(build_class_body(&mut dummy_tokens.iter().peekable(), "_class_name"), result_xml);
    // }
    
    // Class-var-TESTS
    // #[test]
    // fn only_static_and_field_are_class_vars() {
    //     let field_keyword = Token { token_type: TokenType::Keyword, value: String::from("field") };
    //     let static_keyword = Token { token_type: TokenType::Keyword, value: String::from("static") };
    //     let no_class_var_keyword = Token { token_type: TokenType::Keyword, value: String::from("class") };
    //     assert_eq!(is_class_var_start(Some(&&field_keyword)), true);
    //     assert_eq!(is_class_var_start(Some(&&static_keyword)), true);
    //     assert_eq!(is_class_var_start(Some(&&no_class_var_keyword)), false);
    // }
    
    
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
    // #[test]
    // fn subroutine_compiled() {
    //     let dummy_subroutine = "\
    //         function char myfunc(int age, boolean isCool) {\
    //             var char letter;\
    //             var int max, min;\
    //         }";
    //     let mock_body_tokens = tokenize(dummy_subroutine);
    //     // Update subroutine-table
    //     let TODO = compile_subroutine(&mut mock_body_tokens.iter().peekable(), "_class_name", &ClassTable::default());
    //     assert_eq!(vec!["TODO".to_string()], TODO);
    // }
    
    // // Subroutine-Body-TESTS
    // #[test]
    // fn subroutine_body_updates_local_var_table() {
    //     let mock_body = "\
    //         var int num, count;\
    //         var boolean isOpen;\
    //     }";
    //     let mut dummy_subroutine_table = SubroutineTable::default();
    //     let mock_body_tokens = tokenize(mock_body);
    //     // Update subroutine-table
    //     compile_subroutine_body(&mut mock_body_tokens.iter().peekable(), &ClassTable::default(), &mut dummy_subroutine_table);
    //     assert_eq!(Some(Var {kind: "local".to_string(), typ: "int".to_string(), idx: 0}), dummy_subroutine_table.get("num"));
    //     assert_eq!(Some(Var {kind: "local".to_string(), typ: "count".to_string(), idx: 1}), dummy_subroutine_table.get("isOpen"));
    // }
    
    
    // // ### Statement TESTS ###
    
    // // Complex Statement body Tests
    // #[test]
    // fn statement_body_compiles() {
    //     let dummy_statement_body = "\
    //         if (true) {\
    //             while (false) {\
    //             }\
    //         } else {\
    //             do rockit();\
    //         }\
    //     }";
    //     let dummy_statement_body_tokens = tokenize(dummy_statement_body);
    //     let dummy_statement_body_xml = vec![String::from("TODO")];
    //     assert_eq!(compile_statement_body(&mut dummy_statement_body_tokens.iter().peekable(), &ClassTable::default(), &SubroutineTable::default()), dummy_statement_body_xml);
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
    // #[test]
    // fn do_subroutine_compiles() {
    //     let dummy_do = "do rockit(x);";
    //     let dummy_do_tokens = tokenize(dummy_do);
    //     let dummy_do_xml = vec![String::from("TODO")];
    //         assert_eq!(compile_do(&mut dummy_do_tokens.iter().peekable(), &ClassTable::default(), &SubroutineTable::default()), dummy_do_xml);
    // }
    // #[test]
    // fn do_multiple_expressions_compiles() {
    //     let dummy_do = "do rockit(x, y, z);";
    //     let dummy_do_tokens = tokenize(dummy_do);
    //     let dummy_do_xml = vec![String::from("TODO")];
    //         assert_eq!(compile_do(&mut dummy_do_tokens.iter().peekable(), &ClassTable::default(), &SubroutineTable::default()), dummy_do_xml);
    // }
        
    // #[test]
    // fn do_class_dot_subroutine_compiles() {
    //     let dummy_do = "do Great.rockit(x);";
    //     let dummy_do_tokens = tokenize(dummy_do);
    //     let dummy_do_xml = vec![String::from("TODO")];
    //     assert_eq!(compile_do(&mut dummy_do_tokens.iter().peekable(), &ClassTable::default(), &SubroutineTable::default()), dummy_do_xml);
    // }
    
    // //Return - TESTS
    // #[test]
    // fn return_without_expression_compiles() {
    //     let dummy_return_tokens = tokenize("return;");
    //     let dummy_return = vec!["return".to_string()];
    //     assert_eq!(compile_return(&mut dummy_return_tokens.iter().peekable(), &ClassTable::default(), &SubroutineTable::default()), dummy_return);
    // }
    // #[test]
    // fn return_with_expression_compiles() {
    //     let dummy_return_tokens = tokenize(r#"return "cool";"#);
    //     let dummy_return = vec![String::from("TODO")];
    //     assert_eq!(compile_return(&mut dummy_return_tokens.iter().peekable(), &ClassTable::default(), &SubroutineTable::default()), dummy_return);
    // }
    
    // // ### Expressions TESTS ###
    // #[test]
    // fn expressionless_compiles() {
    //     let dummy_exp_tokens = tokenize("x;");
    //     let dummy_var = Var::new("local", "int", 0);
    //     let mut dummy_subroutine_table = SubroutineTable::default();
    //     dummy_subroutine_table.add("local", dummy_var);
    //     let dummy_exp = vec![String::from("push local 0")];
    //     assert_eq!(compile_expression(&mut dummy_exp_tokens.iter().peekable(), &mut ClassTable::default(), &mut dummy_subroutine_table), dummy_exp);
    // }
    
    // // term - TESTS (part of expression)
    // #[test]
    // fn term_compiles() {
    //     let dummy_term_tokens = tokenize("1 / 2;");
    //     let dummy_term = vec!["push constant 1".to_string(), "push constant 2".to_string(), "call Math.devide 2".to_string()];
    //     assert_eq!(compile_term(&mut dummy_term_tokens.iter().peekable(), &mut ClassTable::default(), &mut SubroutineTable::default()), dummy_term);
    // }
//}
