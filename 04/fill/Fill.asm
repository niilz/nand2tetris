// This file is part of www.nand2tetris.org
// and the book "The Elements of Computing Systems"
// by Nisan and Schocken, MIT Press.
// File name: projects/04/Fill.asm

// Runs an infinite loop that listens to the keyboard input.
// When a key is pressed (any key), the program blackens the screen,
// i.e. writes "black" in every pixel;
// the screen should remain fully black as long as the key is pressed. 
// When no key is pressed, the program clears the screen, i.e. writes
// "white" in every pixel;
// the screen should remain fully clear as long as no key is pressed.

// Put your code here.

@SCREEN
D = A           // get SCREEN-Addr

@PixelAddr      // save current SCREEN-Addr in varible PIXELADDR
M = D

@24576            // define a max value of SCREEN-Addr
D = A
@max
M = D 

(Loop)           // start of the painting loop
    @KBD         // Check Keyboarvalue
    D = M

    @Black       // if Keyboardvalue != 0 go to Blacken the screen
    D; JNE
    @White
    D; JEQ

    

(Black)
    @PixelAddr   // get variable with current SCREEN-Addr
    A = M        // set ADDRESS to CURRENT-SCREEN-Addr
    M = -1       // turn it's value to -1 (black) if KBD pressed else to 0
    @Draw
    0; JMP

(White)
    @PixelAddr   // get variable with current SCREEN-Addr
    A = M        // set ADDRESS to CURRENT-SCREEN-Addr
    M = 0        // make pixels white

(Draw)
    @PixelAddr   // increase current SCREEN-Addr by one
    M = M+1
    
    D = M        // put that ADDRESS value into D
    @max         // substract D from MAX-SCREEN-VALUE
    D = M - D
    @Loop         // Jump to beginning of loop if D is greater zeor (max SCREEN-Addr has not been reached yet)
    D; JGT

    @SCREEN      // screen is full, so set PixelAddr back to beginning
    D = A
    @PixelAddr
    M = D
    @Loop           
    0; JMP    