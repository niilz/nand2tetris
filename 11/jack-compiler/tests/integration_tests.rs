mod commons;
use jack_compiler::tokenizer::{tokenize};
use jack_compiler::compiler::*;

#[test]
fn do_integration_seven() {
    let supposed_vm_output = commons::get_supposed_vm_output_seven();
    let actual_vm_output = commons::compile_file_and_get_output("Seven", "Main");
    assert_eq!(supposed_vm_output, actual_vm_output);
}
#[test]
fn do_integration_square() {
    let supposed_vm_output = commons::get_supposed_vm_output_square();
    let actual_vm_output = commons::compile_file_and_get_output("Square", "Square");
    assert_eq!(supposed_vm_output, actual_vm_output);
}
#[test]
fn do_integration_average() {
    let supposed_vm_output = commons::get_supposed_vm_output_average();
    let actual_vm_output = commons::compile_file_and_get_output("Average", "Main");
    assert_eq!(supposed_vm_output, actual_vm_output);
}
#[test]
fn do_integration_pong() {
    let supposed_vm_output = commons::get_supposed_vm_output_pong();
    let actual_vm_output = commons::compile_file_and_get_output("Pong", "PongGame");
    assert_eq!(supposed_vm_output, actual_vm_output);
}

// LET TESTS
// Let Statement-TEST
#[test]
fn let_wihtout_expression_compiles() {
    let dummy_let_tokens = tokenize("let first = 50;");
    let mut compiler = Compiler::new(&dummy_let_tokens, "Noclass");
    let dummy_let = vec![String::from("TODO")];
    assert_eq!(compiler.compile_let(), dummy_let);
}
#[test]
fn let_with_or_compiles() {
    let dummy_let_tokens = tokenize("let second = 50 | 60;");
    let mut compiler = Compiler::new(&dummy_let_tokens, "Noclass");
    let dummy_let = vec![String::from("TODO")];
    assert_eq!(compiler.compile_let(), dummy_let);
}

#[test]
fn let_with_array_idx_compiles() {
    let dummy_let_tokens = tokenize("let myVar[i] = 50;");
    let mut compiler = Compiler::new(&dummy_let_tokens, "Noclass");
    let dummy_let = vec![String::from("TODO")];
    assert_eq!(compiler.compile_let(), dummy_let);
}
#[test]
fn let_subroutine_call_compiles() {
    let dummy_let_tokens = tokenize("let subR = myFunc.call();");
    let mut compiler = Compiler::new(&dummy_let_tokens, "Noclass");
    let dummy_let = vec![String::from("TODO")];
    assert_eq!(compiler.compile_let(), dummy_let);
}
#[test]
fn let_array_idx_compiles() {
    let dummy_let_tokens = tokenize("let a[1]= blup;");
    let mut compiler = Compiler::new(&dummy_let_tokens, "Noclass");
    let dummy_let = vec![String::from("TODO")];
    assert_eq!(compiler.compile_let(), dummy_let);
}
#[test]
fn let_with_parantheses_compiles() {
    let dummy_let_tokens = tokenize("let b = c * (-3);");
    let mut compiler = Compiler::new(&dummy_let_tokens, "Noclass");
    let dummy_let = vec![String::from("TODO")];
    assert_eq!(compiler.compile_let(), dummy_let);
}
#[test]
fn let_with_square_term_right_compiles() {
    let dummy_let_tokens = tokenize("let sum = sum + a[i];");
    let mut compiler = Compiler::new(&dummy_let_tokens, "Noclass");
    let dummy_let = vec![String::from("TODO")];
    assert_eq!(compiler.compile_let(), dummy_let);
}