use jack_compiler::processing::{ process_input };
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

pub fn get_supposed_vm_output_average() -> String {
r"// ByteCode for class 'Main'


function Main.main 4

push constant 18
call String.new 1
push constant 72
call String.appendChar 2
push constant 111
call String.appendChar 2
push constant 119
call String.appendChar 2
push constant 32
call String.appendChar 2
push constant 109
call String.appendChar 2
push constant 97
call String.appendChar 2
push constant 110
call String.appendChar 2
push constant 121
call String.appendChar 2
push constant 32
call String.appendChar 2
push constant 110
call String.appendChar 2
push constant 117
call String.appendChar 2
push constant 109
call String.appendChar 2
push constant 98
call String.appendChar 2
push constant 101
call String.appendChar 2
push constant 114
call String.appendChar 2
push constant 115
call String.appendChar 2
push constant 63
call String.appendChar 2
push constant 32
call String.appendChar 2
call Keyboard.readInt 1
pop local 1
push local 1
call Array.new 1
pop local 0
push constant 0
pop local 2
label Main.main$0.WHILESTART
push local 2
push local 1
lt
not
if-goto Main.main$0.WHILEEND
push local 0
push local 2
add
push constant 16
call String.new 1
push constant 69
call String.appendChar 2
push constant 110
call String.appendChar 2
push constant 116
call String.appendChar 2
push constant 101
call String.appendChar 2
push constant 114
call String.appendChar 2
push constant 32
call String.appendChar 2
push constant 97
call String.appendChar 2
push constant 32
call String.appendChar 2
push constant 110
call String.appendChar 2
push constant 117
call String.appendChar 2
push constant 109
call String.appendChar 2
push constant 98
call String.appendChar 2
push constant 101
call String.appendChar 2
push constant 114
call String.appendChar 2
push constant 58
call String.appendChar 2
push constant 32
call String.appendChar 2
call Keyboard.readInt 1
pop temp 0
pop pointer 1
push temp 0
pop that 0
push local 3
push local 0
push local 2
add
pop pointer 1
push that 0
add
pop local 3
push local 2
push constant 1
add
pop local 2
goto Main.main$0.WHILESTART
label Main.main$0.WHILEEND
push constant 15
call String.new 1
push constant 84
call String.appendChar 2
push constant 104
call String.appendChar 2
push constant 101
call String.appendChar 2
push constant 32
call String.appendChar 2
push constant 97
call String.appendChar 2
push constant 118
call String.appendChar 2
push constant 101
call String.appendChar 2
push constant 114
call String.appendChar 2
push constant 97
call String.appendChar 2
push constant 103
call String.appendChar 2
push constant 101
call String.appendChar 2
push constant 32
call String.appendChar 2
push constant 105
call String.appendChar 2
push constant 115
call String.appendChar 2
push constant 32
call String.appendChar 2
call Output.printString 1
pop temp 0
push local 3
push local 1
call Math.divide 2
call Output.printInt 1
pop temp 0
push constant 0
return
".to_string()
}

pub fn get_supposed_vm_output_pong() -> String {
r"// ByteCode for class 'PongGame'

// Class-Var-Dec Fields: #0, Statics: #1

// Class-Var-Dec Fields: #1, Statics: #1

// Class-Var-Dec Fields: #2, Statics: #1

// Class-Var-Dec Fields: #3, Statics: #1

// Class-Var-Dec Fields: #4, Statics: #1

// Class-Var-Dec Fields: #5, Statics: #1

// Class-Var-Dec Fields: #6, Statics: #1

// Class-Var-Dec Fields: #7, Statics: #1


function PongGame.new 0

push constant 7
call Memory.alloc 1
pop pointer 0
call Screen.clearScreen 0
pop temp 0
push constant 50
pop this 6
push constant 230
push constant 229
push this 6
push constant 7
call Bat.new 4
pop this 0
push constant 253
push constant 222
push constant 0
push constant 511
push constant 0
push constant 229
call Ball.new 6
pop this 1
push this 1
push constant 400
push constant 0
call Ball.setDestination 3
pop temp 0
push constant 0
push constant 238
push constant 511
push constant 240
call Screen.drawRectangle 4
pop temp 0
push constant 22
push constant 0
call Output.moveCursor 2
pop temp 0
push constant 8
call String.new 1
push constant 83
call String.appendChar 2
push constant 99
call String.appendChar 2
push constant 111
call String.appendChar 2
push constant 114
call String.appendChar 2
push constant 101
call String.appendChar 2
push constant 58
call String.appendChar 2
push constant 32
call String.appendChar 2
push constant 48
call String.appendChar 2
call Output.printString 1
pop temp 0
push constant 0
pop this 3
push constant 0
pop this 4
push constant 0
pop this 2
push constant 0
pop this 5
push pointer 0
// non_void_no_dummy_0
return


function PongGame.dispose 0

push argument 0
pop pointer 0
push this 0
call Bat.dispose 1
pop temp 0
push this 1
call Ball.dispose 1
pop temp 0
push pointer 0
call Memory.deAlloc 1
pop temp 0
push constant 0
return


function PongGame.newInstance 0

call PongGame.new 0
pop static 0
push constant 0
return


function PongGame.getInstance 0

push static 0
// non_void_no_dummy_0
return


function PongGame.run 1

push argument 0
pop pointer 0
label PongGame.run$0.WHILESTART
push this 3
not
not
if-goto PongGame.run$0.WHILEEND
label PongGame.run$1.WHILESTART
push local 0
push constant 0
eq
push this 3
not
and
not
if-goto PongGame.run$1.WHILEEND
call Keyboard.keyPressed 0
pop local 0
push this 0
call Bat.move 1
pop temp 0
push pointer 0
call PongGame.moveBall 1
pop temp 0
push constant 50
call Sys.wait 1
pop temp 0
goto PongGame.run$1.WHILESTART
label PongGame.run$1.WHILEEND
label PongGame.run$2.IFSTART
push local 0
push constant 130
eq
not
if-goto PongGame.run$2.ELSESTART
push this 0
push constant 1
call Bat.setDirection 2
pop temp 0
goto PongGame.run$2.IFEND
label PongGame.run$2.ELSESTART
label PongGame.run$3.IFSTART
push local 0
push constant 132
eq
not
if-goto PongGame.run$3.ELSESTART
push this 0
push constant 2
call Bat.setDirection 2
pop temp 0
goto PongGame.run$3.IFEND
label PongGame.run$3.ELSESTART
label PongGame.run$4.IFSTART
push local 0
push constant 140
eq
not
if-goto PongGame.run$4.ELSESTART
push constant 1
neg
pop this 3
goto PongGame.run$4.IFEND
label PongGame.run$4.ELSESTART
label PongGame.run$4.IFEND
label PongGame.run$3.IFEND
label PongGame.run$2.IFEND
label PongGame.run$5.WHILESTART
push local 0
push constant 0
eq
not
push this 3
not
and
not
if-goto PongGame.run$5.WHILEEND
call Keyboard.keyPressed 0
pop local 0
push this 0
call Bat.move 1
pop temp 0
push pointer 0
call PongGame.moveBall 1
pop temp 0
push constant 50
call Sys.wait 1
pop temp 0
goto PongGame.run$5.WHILESTART
label PongGame.run$5.WHILEEND
goto PongGame.run$0.WHILESTART
label PongGame.run$0.WHILEEND
label PongGame.run$6.IFSTART
push this 3
not
if-goto PongGame.run$6.ELSESTART
push constant 10
push constant 27
call Output.moveCursor 2
pop temp 0
push constant 9
call String.new 1
push constant 71
call String.appendChar 2
push constant 97
call String.appendChar 2
push constant 109
call String.appendChar 2
push constant 101
call String.appendChar 2
push constant 32
call String.appendChar 2
push constant 79
call String.appendChar 2
push constant 118
call String.appendChar 2
push constant 101
call String.appendChar 2
push constant 114
call String.appendChar 2
call Output.printString 1
pop temp 0
goto PongGame.run$6.IFEND
label PongGame.run$6.ELSESTART
label PongGame.run$6.IFEND
push constant 0
return


function PongGame.moveBall 5

push argument 0
pop pointer 0
push this 1
call Ball.move 1
pop this 2
label PongGame.moveBall$7.IFSTART
push this 2
push constant 0
gt
push this 2
push this 5
eq
not
and
not
if-goto PongGame.moveBall$7.ELSESTART
push this 2
pop this 5
push constant 0
pop local 0
push this 0
call Bat.getLeft 1
pop local 1
push this 0
call Bat.getRight 1
pop local 2
push this 1
call Ball.getLeft 1
pop local 3
push this 1
call Ball.getRight 1
pop local 4
label PongGame.moveBall$8.IFSTART
push this 2
push constant 4
eq
not
if-goto PongGame.moveBall$8.ELSESTART
push local 1
push local 4
gt
push local 2
push local 3
lt
or
pop this 3
label PongGame.moveBall$9.IFSTART
push this 3
not
not
if-goto PongGame.moveBall$9.ELSESTART
label PongGame.moveBall$10.IFSTART
push local 4
push local 1
push constant 10
add
lt
not
if-goto PongGame.moveBall$10.ELSESTART
push constant 1
neg
pop local 0
goto PongGame.moveBall$10.IFEND
label PongGame.moveBall$10.ELSESTART
label PongGame.moveBall$11.IFSTART
push local 3
push local 2
push constant 10
sub
gt
not
if-goto PongGame.moveBall$11.ELSESTART
push constant 1
pop local 0
goto PongGame.moveBall$11.IFEND
label PongGame.moveBall$11.ELSESTART
label PongGame.moveBall$11.IFEND
label PongGame.moveBall$10.IFEND
push this 6
push constant 2
sub
pop this 6
push this 0
push this 6
call Bat.setWidth 2
pop temp 0
push this 4
push constant 1
add
pop this 4
push constant 22
push constant 7
call Output.moveCursor 2
pop temp 0
push this 4
call Output.printInt 1
pop temp 0
goto PongGame.moveBall$9.IFEND
label PongGame.moveBall$9.ELSESTART
label PongGame.moveBall$9.IFEND
goto PongGame.moveBall$8.IFEND
label PongGame.moveBall$8.ELSESTART
label PongGame.moveBall$8.IFEND
push this 1
push local 0
call Ball.bounce 2
pop temp 0
goto PongGame.moveBall$7.IFEND
label PongGame.moveBall$7.ELSESTART
label PongGame.moveBall$7.IFEND
push constant 0
return
".to_string()
}

pub fn get_supposed_vm_output_complex_arrays() -> String {
r"// ByteCode for class 'Main'


function Main.main 3

push constant 10
call Array.new 1
pop local 0
push constant 5
call Array.new 1
pop local 1
push constant 1
call Array.new 1
pop local 2
push local 0
push constant 3
add
push constant 2
pop temp 0
pop pointer 1
push temp 0
pop that 0
push local 0
push constant 4
add
push constant 8
pop temp 0
pop pointer 1
push temp 0
pop that 0
push local 0
push constant 5
add
push constant 4
pop temp 0
pop pointer 1
push temp 0
pop that 0
push local 1
push local 0
push constant 3
add
pop pointer 1
push that 0
add
push local 0
push constant 3
add
pop pointer 1
push that 0
push constant 3
add
pop temp 0
pop pointer 1
push temp 0
pop that 0
push local 0
push local 1
push local 0
push constant 3
add
pop pointer 1
push that 0
add
pop pointer 1
push that 0
add
push local 0
push local 0
push constant 5
add
pop pointer 1
push that 0
add
pop pointer 1
push that 0
push local 1
push constant 7
push local 0
push constant 3
add
pop pointer 1
push that 0
sub
push constant 2
call Main.double 1
sub
push constant 1
add
add
pop pointer 1
push that 0
call Math.multiply 2
pop temp 0
pop pointer 1
push temp 0
pop that 0
push local 2
push constant 0
add
push constant 0
pop temp 0
pop pointer 1
push temp 0
pop that 0
push local 2
push constant 0
add
pop pointer 1
push that 0
pop local 2
push constant 43
call String.new 1
push constant 84
call String.appendChar 2
push constant 101
call String.appendChar 2
push constant 115
call String.appendChar 2
push constant 116
call String.appendChar 2
push constant 32
call String.appendChar 2
push constant 49
call String.appendChar 2
push constant 58
call String.appendChar 2
push constant 32
call String.appendChar 2
push constant 101
call String.appendChar 2
push constant 120
call String.appendChar 2
push constant 112
call String.appendChar 2
push constant 101
call String.appendChar 2
push constant 99
call String.appendChar 2
push constant 116
call String.appendChar 2
push constant 101
call String.appendChar 2
push constant 100
call String.appendChar 2
push constant 32
call String.appendChar 2
push constant 114
call String.appendChar 2
push constant 101
call String.appendChar 2
push constant 115
call String.appendChar 2
push constant 117
call String.appendChar 2
push constant 108
call String.appendChar 2
push constant 116
call String.appendChar 2
push constant 58
call String.appendChar 2
push constant 32
call String.appendChar 2
push constant 53
call String.appendChar 2
push constant 59
call String.appendChar 2
push constant 32
call String.appendChar 2
push constant 97
call String.appendChar 2
push constant 99
call String.appendChar 2
push constant 116
call String.appendChar 2
push constant 117
call String.appendChar 2
push constant 97
call String.appendChar 2
push constant 108
call String.appendChar 2
push constant 32
call String.appendChar 2
push constant 114
call String.appendChar 2
push constant 101
call String.appendChar 2
push constant 115
call String.appendChar 2
push constant 117
call String.appendChar 2
push constant 108
call String.appendChar 2
push constant 116
call String.appendChar 2
push constant 58
call String.appendChar 2
push constant 32
call String.appendChar 2
call Output.printString 1
pop temp 0
push local 1
push constant 2
add
pop pointer 1
push that 0
call Output.printInt 1
pop temp 0
call Output.println 0
pop temp 0
push constant 44
call String.new 1
push constant 84
call String.appendChar 2
push constant 101
call String.appendChar 2
push constant 115
call String.appendChar 2
push constant 116
call String.appendChar 2
push constant 32
call String.appendChar 2
push constant 50
call String.appendChar 2
push constant 58
call String.appendChar 2
push constant 32
call String.appendChar 2
push constant 101
call String.appendChar 2
push constant 120
call String.appendChar 2
push constant 112
call String.appendChar 2
push constant 101
call String.appendChar 2
push constant 99
call String.appendChar 2
push constant 116
call String.appendChar 2
push constant 101
call String.appendChar 2
push constant 100
call String.appendChar 2
push constant 32
call String.appendChar 2
push constant 114
call String.appendChar 2
push constant 101
call String.appendChar 2
push constant 115
call String.appendChar 2
push constant 117
call String.appendChar 2
push constant 108
call String.appendChar 2
push constant 116
call String.appendChar 2
push constant 58
call String.appendChar 2
push constant 32
call String.appendChar 2
push constant 52
call String.appendChar 2
push constant 48
call String.appendChar 2
push constant 59
call String.appendChar 2
push constant 32
call String.appendChar 2
push constant 97
call String.appendChar 2
push constant 99
call String.appendChar 2
push constant 116
call String.appendChar 2
push constant 117
call String.appendChar 2
push constant 97
call String.appendChar 2
push constant 108
call String.appendChar 2
push constant 32
call String.appendChar 2
push constant 114
call String.appendChar 2
push constant 101
call String.appendChar 2
push constant 115
call String.appendChar 2
push constant 117
call String.appendChar 2
push constant 108
call String.appendChar 2
push constant 116
call String.appendChar 2
push constant 58
call String.appendChar 2
push constant 32
call String.appendChar 2
call Output.printString 1
pop temp 0
push local 0
push constant 5
add
pop pointer 1
push that 0
call Output.printInt 1
pop temp 0
call Output.println 0
pop temp 0
push constant 43
call String.new 1
push constant 84
call String.appendChar 2
push constant 101
call String.appendChar 2
push constant 115
call String.appendChar 2
push constant 116
call String.appendChar 2
push constant 32
call String.appendChar 2
push constant 51
call String.appendChar 2
push constant 58
call String.appendChar 2
push constant 32
call String.appendChar 2
push constant 101
call String.appendChar 2
push constant 120
call String.appendChar 2
push constant 112
call String.appendChar 2
push constant 101
call String.appendChar 2
push constant 99
call String.appendChar 2
push constant 116
call String.appendChar 2
push constant 101
call String.appendChar 2
push constant 100
call String.appendChar 2
push constant 32
call String.appendChar 2
push constant 114
call String.appendChar 2
push constant 101
call String.appendChar 2
push constant 115
call String.appendChar 2
push constant 117
call String.appendChar 2
push constant 108
call String.appendChar 2
push constant 116
call String.appendChar 2
push constant 58
call String.appendChar 2
push constant 32
call String.appendChar 2
push constant 48
call String.appendChar 2
push constant 59
call String.appendChar 2
push constant 32
call String.appendChar 2
push constant 97
call String.appendChar 2
push constant 99
call String.appendChar 2
push constant 116
call String.appendChar 2
push constant 117
call String.appendChar 2
push constant 97
call String.appendChar 2
push constant 108
call String.appendChar 2
push constant 32
call String.appendChar 2
push constant 114
call String.appendChar 2
push constant 101
call String.appendChar 2
push constant 115
call String.appendChar 2
push constant 117
call String.appendChar 2
push constant 108
call String.appendChar 2
push constant 116
call String.appendChar 2
push constant 58
call String.appendChar 2
push constant 32
call String.appendChar 2
call Output.printString 1
pop temp 0
push local 2
call Output.printInt 1
pop temp 0
call Output.println 0
pop temp 0
push constant 0
pop local 2
label Main.main$0.IFSTART
push local 2
push constant 0
eq
not
if-goto Main.main$0.ELSESTART
push local 0
push constant 10
call Main.fill 2
pop temp 0
push local 0
push constant 3
add
pop pointer 1
push that 0
pop local 2
push local 2
push constant 1
add
push constant 33
pop temp 0
pop pointer 1
push temp 0
pop that 0
push local 0
push constant 7
add
pop pointer 1
push that 0
pop local 2
push local 2
push constant 1
add
push constant 77
pop temp 0
pop pointer 1
push temp 0
pop that 0
push local 0
push constant 3
add
pop pointer 1
push that 0
pop local 1
push local 1
push constant 1
add
push local 1
push constant 1
add
pop pointer 1
push that 0
push local 2
push constant 1
add
pop pointer 1
push that 0
add
pop temp 0
pop pointer 1
push temp 0
pop that 0
goto Main.main$0.IFEND
label Main.main$0.ELSESTART
label Main.main$0.IFEND
push constant 44
call String.new 1
push constant 84
call String.appendChar 2
push constant 101
call String.appendChar 2
push constant 115
call String.appendChar 2
push constant 116
call String.appendChar 2
push constant 32
call String.appendChar 2
push constant 52
call String.appendChar 2
push constant 58
call String.appendChar 2
push constant 32
call String.appendChar 2
push constant 101
call String.appendChar 2
push constant 120
call String.appendChar 2
push constant 112
call String.appendChar 2
push constant 101
call String.appendChar 2
push constant 99
call String.appendChar 2
push constant 116
call String.appendChar 2
push constant 101
call String.appendChar 2
push constant 100
call String.appendChar 2
push constant 32
call String.appendChar 2
push constant 114
call String.appendChar 2
push constant 101
call String.appendChar 2
push constant 115
call String.appendChar 2
push constant 117
call String.appendChar 2
push constant 108
call String.appendChar 2
push constant 116
call String.appendChar 2
push constant 58
call String.appendChar 2
push constant 32
call String.appendChar 2
push constant 55
call String.appendChar 2
push constant 55
call String.appendChar 2
push constant 59
call String.appendChar 2
push constant 32
call String.appendChar 2
push constant 97
call String.appendChar 2
push constant 99
call String.appendChar 2
push constant 116
call String.appendChar 2
push constant 117
call String.appendChar 2
push constant 97
call String.appendChar 2
push constant 108
call String.appendChar 2
push constant 32
call String.appendChar 2
push constant 114
call String.appendChar 2
push constant 101
call String.appendChar 2
push constant 115
call String.appendChar 2
push constant 117
call String.appendChar 2
push constant 108
call String.appendChar 2
push constant 116
call String.appendChar 2
push constant 58
call String.appendChar 2
push constant 32
call String.appendChar 2
call Output.printString 1
pop temp 0
push local 2
push constant 1
add
pop pointer 1
push that 0
call Output.printInt 1
pop temp 0
call Output.println 0
pop temp 0
push constant 45
call String.new 1
push constant 84
call String.appendChar 2
push constant 101
call String.appendChar 2
push constant 115
call String.appendChar 2
push constant 116
call String.appendChar 2
push constant 32
call String.appendChar 2
push constant 53
call String.appendChar 2
push constant 58
call String.appendChar 2
push constant 32
call String.appendChar 2
push constant 101
call String.appendChar 2
push constant 120
call String.appendChar 2
push constant 112
call String.appendChar 2
push constant 101
call String.appendChar 2
push constant 99
call String.appendChar 2
push constant 116
call String.appendChar 2
push constant 101
call String.appendChar 2
push constant 100
call String.appendChar 2
push constant 32
call String.appendChar 2
push constant 114
call String.appendChar 2
push constant 101
call String.appendChar 2
push constant 115
call String.appendChar 2
push constant 117
call String.appendChar 2
push constant 108
call String.appendChar 2
push constant 116
call String.appendChar 2
push constant 58
call String.appendChar 2
push constant 32
call String.appendChar 2
push constant 49
call String.appendChar 2
push constant 49
call String.appendChar 2
push constant 48
call String.appendChar 2
push constant 59
call String.appendChar 2
push constant 32
call String.appendChar 2
push constant 97
call String.appendChar 2
push constant 99
call String.appendChar 2
push constant 116
call String.appendChar 2
push constant 117
call String.appendChar 2
push constant 97
call String.appendChar 2
push constant 108
call String.appendChar 2
push constant 32
call String.appendChar 2
push constant 114
call String.appendChar 2
push constant 101
call String.appendChar 2
push constant 115
call String.appendChar 2
push constant 117
call String.appendChar 2
push constant 108
call String.appendChar 2
push constant 116
call String.appendChar 2
push constant 58
call String.appendChar 2
push constant 32
call String.appendChar 2
call Output.printString 1
pop temp 0
push local 1
push constant 1
add
pop pointer 1
push that 0
call Output.printInt 1
pop temp 0
call Output.println 0
pop temp 0
push constant 0
return


function Main.double 0

push argument 0
push constant 2
call Math.multiply 2
// non_void_no_dummy_0
return


function Main.fill 0

label Main.fill$1.WHILESTART
push argument 1
push constant 0
gt
not
if-goto Main.fill$1.WHILEEND
push argument 1
push constant 1
sub
pop argument 1
push argument 0
push argument 1
add
push constant 3
call Array.new 1
pop temp 0
pop pointer 1
push temp 0
pop that 0
goto Main.fill$1.WHILESTART
label Main.fill$1.WHILEEND
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