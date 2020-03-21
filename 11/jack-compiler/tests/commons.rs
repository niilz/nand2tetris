use jack_compiler::processing::{ process_input };
use jack_compiler::compiler::tables::{ ClassTable, SubroutineTable, Var };
use std::path::{ Path };
use std::fs;

pub fn compile_file_and_get_output(folder: &str, file: &str) -> String {
    let path_string = format!("../../11/{}", folder);
    let path = Path::new(&path_string);
    // Compiles jack-file(s) in dir and writes it into file.vm
    process_input(path);
    let result_path_string = format!("../../11/{}/{}.vm", folder, file);
    let result_file_path = Path::new(&result_path_string);
    // Reads back the compiled output
    let compiled_result = fs::read_to_string(result_file_path).expect("could not read result vm-file in test");
    
    compiled_result
}

pub fn get_supposed_vm_output_seven() -> String {
r"// ByteCode for class 'Main'


function Main.main 0

push constant 1
push constant 2
push constant 3
call Math.multiply 2
add
call Output.printInt 1
pop temp 0
push constant 0
return
".to_string()
}

pub fn get_supposed_vm_output_square() -> String {
r"// ByteCode for class 'Square'

// Class-Var-Dec Fields: #2, Statics: #0

// Class-Var-Dec Fields: #3, Statics: #0


function Square.new 0

push constant 3
call Memory.alloc 1
pop pointer 0
push argument 0
pop this 0
push argument 1
pop this 1
push argument 2
pop this 2
push pointer 0
call Square.draw 1
pop temp 0
push pointer 0
// non_void_no_dummy_0
return


function Square.dispose 0

push argument 0
pop pointer 0
push pointer 0
call Memory.deAlloc 1
pop temp 0
push constant 0
return


function Square.draw 0

push argument 0
pop pointer 0
push constant 1
neg
call Screen.setColor 1
pop temp 0
push this 0
push this 1
push this 0
push this 2
add
push this 1
push this 2
add
call Screen.drawRectangle 4
pop temp 0
push constant 0
return


function Square.erase 0

push argument 0
pop pointer 0
push constant 0
call Screen.setColor 1
pop temp 0
push this 0
push this 1
push this 0
push this 2
add
push this 1
push this 2
add
call Screen.drawRectangle 4
pop temp 0
push constant 0
return


function Square.incSize 0

push argument 0
pop pointer 0
label Square.incSize$0.IFSTART
push this 1
push this 2
add
push constant 254
lt
push this 0
push this 2
add
push constant 510
lt
and
not
if-goto Square.incSize$0.ELSESTART
push pointer 0
call Square.erase 1
pop temp 0
push this 2
push constant 2
add
pop this 2
push pointer 0
call Square.draw 1
pop temp 0
goto Square.incSize$0.IFEND
label Square.incSize$0.ELSESTART
label Square.incSize$0.IFEND
push constant 0
return


function Square.decSize 0

push argument 0
pop pointer 0
label Square.decSize$1.IFSTART
push this 2
push constant 2
gt
not
if-goto Square.decSize$1.ELSESTART
push pointer 0
call Square.erase 1
pop temp 0
push this 2
push constant 2
sub
pop this 2
push pointer 0
call Square.draw 1
pop temp 0
goto Square.decSize$1.IFEND
label Square.decSize$1.ELSESTART
label Square.decSize$1.IFEND
push constant 0
return


function Square.moveUp 0

push argument 0
pop pointer 0
label Square.moveUp$2.IFSTART
push this 1
push constant 1
gt
not
if-goto Square.moveUp$2.ELSESTART
push constant 0
call Screen.setColor 1
pop temp 0
push this 0
push this 1
push this 2
add
push constant 1
sub
push this 0
push this 2
add
push this 1
push this 2
add
call Screen.drawRectangle 4
pop temp 0
push this 1
push constant 2
sub
pop this 1
push constant 1
neg
call Screen.setColor 1
pop temp 0
push this 0
push this 1
push this 0
push this 2
add
push this 1
push constant 1
add
call Screen.drawRectangle 4
pop temp 0
goto Square.moveUp$2.IFEND
label Square.moveUp$2.ELSESTART
label Square.moveUp$2.IFEND
push constant 0
return


function Square.moveDown 0

push argument 0
pop pointer 0
label Square.moveDown$3.IFSTART
push this 1
push this 2
add
push constant 254
lt
not
if-goto Square.moveDown$3.ELSESTART
push constant 0
call Screen.setColor 1
pop temp 0
push this 0
push this 1
push this 0
push this 2
add
push this 1
push constant 1
add
call Screen.drawRectangle 4
pop temp 0
push this 1
push constant 2
add
pop this 1
push constant 1
neg
call Screen.setColor 1
pop temp 0
push this 0
push this 1
push this 2
add
push constant 1
sub
push this 0
push this 2
add
push this 1
push this 2
add
call Screen.drawRectangle 4
pop temp 0
goto Square.moveDown$3.IFEND
label Square.moveDown$3.ELSESTART
label Square.moveDown$3.IFEND
push constant 0
return


function Square.moveLeft 0

push argument 0
pop pointer 0
label Square.moveLeft$4.IFSTART
push this 0
push constant 1
gt
not
if-goto Square.moveLeft$4.ELSESTART
push constant 0
call Screen.setColor 1
pop temp 0
push this 0
push this 2
add
push constant 1
sub
push this 1
push this 0
push this 2
add
push this 1
push this 2
add
call Screen.drawRectangle 4
pop temp 0
push this 0
push constant 2
sub
pop this 0
push constant 1
neg
call Screen.setColor 1
pop temp 0
push this 0
push this 1
push this 0
push constant 1
add
push this 1
push this 2
add
call Screen.drawRectangle 4
pop temp 0
goto Square.moveLeft$4.IFEND
label Square.moveLeft$4.ELSESTART
label Square.moveLeft$4.IFEND
push constant 0
return


function Square.moveRight 0

push argument 0
pop pointer 0
label Square.moveRight$5.IFSTART
push this 0
push this 2
add
push constant 510
lt
not
if-goto Square.moveRight$5.ELSESTART
push constant 0
call Screen.setColor 1
pop temp 0
push this 0
push this 1
push this 0
push constant 1
add
push this 1
push this 2
add
call Screen.drawRectangle 4
pop temp 0
push this 0
push constant 2
add
pop this 0
push constant 1
neg
call Screen.setColor 1
pop temp 0
push this 0
push this 2
add
push constant 1
sub
push this 1
push this 0
push this 2
add
push this 1
push this 2
add
call Screen.drawRectangle 4
pop temp 0
goto Square.moveRight$5.IFEND
label Square.moveRight$5.ELSESTART
label Square.moveRight$5.IFEND
push constant 0
return
".to_string()
}

// pub fn get_class_table() -> ClassTable {
//   let mut class_table = ClassTable::default();
//   let var_1 = Var::new("static", "int", 0);
//   class_table.add("first", var_1);
//   let var_2 = Var::new("field", "int", 0);
//   class_table.add("second", var_2);

//   class_table
// }

// pub fn get_subroutine_table() -> SubroutineTable {
//   let mut subroutine_table = SubroutineTable::default();
//   let local_1 = Var::new("local", "int", 0);
//   subroutine_table.add("subR", local_1);
//   let local_2 = Var::new("local", "int", 0);
//   subroutine_table.add("a", local_2);
//   let local_1 = Var::new("arg", "int", 0);
//   subroutine_table.add("b", local_1);
//   let local_2 = Var::new("arg", "int", 0);
//   subroutine_table.add("sum", local_2);

//   subroutine_table
// }