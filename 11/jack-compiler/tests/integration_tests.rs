mod commons;
use jack_compiler::tokenizer::{tokenize};
use jack_compiler::compiler::tables::{ClassTable, SubroutineTable};
use jack_compiler::compiler::*;

#[test]
fn do_integration() {
    let supposed_vm_output = commons::get_supposed_vm_output();
    let actual_vm_output = commons::compile_file_and_get_output();
    assert_eq!(supposed_vm_output, actual_vm_output);
}

// LET TESTS
// Let Statement-TEST
#[test]
fn let_wihtout_expression_compiles() {
    let dummy_let_tokens = tokenize("let first = 50;");
    let class_table = commons::get_class_table();
    let dummy_let = vec![String::from("TODO")];
    assert_eq!(compile_let(&mut dummy_let_tokens.iter().peekable(), &class_table, &SubroutineTable::default()), dummy_let);
}
#[test]
fn let_with_or_compiles() {
    let dummy_let_tokens = tokenize("let second = 50 | 60;");
    let class_table = commons::get_class_table();
    let dummy_let = vec![String::from("TODO")];
    assert_eq!(compile_let(&mut dummy_let_tokens.iter().peekable(), &class_table, &SubroutineTable::default()), dummy_let);
}

#[test]
fn let_with_array_idx_compiles() {
    let dummy_let_tokens = tokenize("let myVar[i] = 50;");
    let subroutine_table = commons::get_subroutine_table();
    let dummy_let = vec![String::from("TODO")];
    assert_eq!(compile_let(&mut dummy_let_tokens.iter().peekable(), &ClassTable::default(), &subroutine_table), dummy_let);
}
#[test]
fn let_subroutine_call_compiles() {
    let dummy_let_tokens = tokenize("let subR = myFunc.call();");
    let subroutine_table = commons::get_subroutine_table();
    let dummy_let = vec![String::from("TODO")];
    assert_eq!(compile_let(&mut dummy_let_tokens.iter().peekable(), &ClassTable::default(), &subroutine_table), dummy_let);
}
#[test]
fn let_array_idx_compiles() {
    let dummy_let_tokens = tokenize("let a[1]= blup;");
    let subroutine_table = commons::get_subroutine_table();
    let dummy_let = vec![String::from("TODO")];
    assert_eq!(compile_let(&mut dummy_let_tokens.iter().peekable(), &ClassTable::default(), &subroutine_table), dummy_let);
}
#[test]
fn let_with_parantheses_compiles() {
    let dummy_let_tokens = tokenize("let b = c * (-3);");
    let subroutine_table = commons::get_subroutine_table();
    let dummy_let = vec![String::from("TODO")];
    assert_eq!(compile_let(&mut dummy_let_tokens.iter().peekable(), &ClassTable::default(), &subroutine_table), dummy_let);
}
#[test]
fn let_with_square_term_right_compiles() {
    let dummy_let_tokens = tokenize("let sum = sum + a[i];");
    let subroutine_table = commons::get_subroutine_table();
    let dummy_let = vec![String::from("TODO")];
    assert_eq!(compile_let(&mut dummy_let_tokens.iter().peekable(), &ClassTable::default(), &subroutine_table), dummy_let);
}