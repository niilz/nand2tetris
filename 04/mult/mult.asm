// This file is part of www.nand2tetris.org
// and the book "The Elements of Computing Systems"
// by Nisan and Schocken, MIT Press.
// File name: projects/04/Mult.asm

// Multiplies R0 and R1 and stores the result in R2.
// (R0, R1, R2 refer to RAM[0], RAM[1], and RAM[2], respectively.)

// Put your code here.

// PSEUDO
// for (i, i < R0, i++) {
//    R2 = R2 + R1
// }

@R2         // set R2 to 0
M = 0

@R0         // load Number in R0
D = M
@End        // if R0 is 0 jump to End (0 time X = 0)
D; JEQ       
@count      // declace a COUNT that holds number of R0
M = D

@R1         // load Number in R1
D = M
@End        // if R1 is 0 jump to End (0 time X = 0)
D; JEQ      
@value      // set a VALUE variable with the number in R1
M = D

@R2         // initalize R2 with 0
M = 0

(Loop)      // start of the LOOP
    @value  // get the consistent VALUE
    D = M
    @R2         // access R2
    M = M + D   // add VALUE to the number in R2

    @count      // get COUNT
    M = M - 1   // decrease COUNT by 1
    D = M       // load that COUNT value into D
    @Loop       // if COUNT is still greater than 0 => jump to the beginning of the LOOP
    D; JGT

(End)           // infinite LOOP to stop the program
    @End
    0; JMP