use jack_compiler::processing::{ process_input };
use jack_compiler::compiler::tables::{ ClassTable, SubroutineTable, Var };
use std::path::{ Path };
use std::fs;

pub fn compile_file_and_get_output() -> String {
    let path = Path::new("../../11/Seven");
    // Compiles jack-file(s) in dir and writes it into file.vm
    process_input(path);
    let result_file_path = Path::new("../../11/Seven/main.vm");
    // Reads back the compiled output
    let compiled_result = fs::read_to_string(result_file_path).expect("could not read result vm-file in test");
    
    compiled_result
}

pub fn get_supposed_vm_output() -> String {
r"// ByteCode for class 'Main'

function Main.main 0

push constant 1
push constant 2
push constant 3
call Math.multiply 2
add
call Output.printInt 1
pop temp 0
return".to_string()
}

pub fn get_class_table() -> ClassTable {
  let mut class_table = ClassTable::default();
  let var_1 = Var::new("static", "int", 0);
  class_table.add("first", var_1);
  let var_2 = Var::new("field", "int", 0);
  class_table.add("second", var_2);

  class_table
}

pub fn get_subroutine_table() -> SubroutineTable {
  let mut subroutine_table = SubroutineTable::default();
  let local_1 = Var::new("local", "int", 0);
  subroutine_table.add("subR", local_1);
  let local_2 = Var::new("local", "int", 0);
  subroutine_table.add("a", local_2);
  let local_1 = Var::new("arg", "int", 0);
  subroutine_table.add("b", local_1);
  let local_2 = Var::new("arg", "int", 0);
  subroutine_table.add("sum", local_2);

  subroutine_table
}