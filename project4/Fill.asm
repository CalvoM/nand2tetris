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

// Pseudocode
// Continuously get the kbd input
// if no input, then run loop from start again
// if no input, make screen white if black is set, and set black to 0;
// if input: turn the screeen black and set black to 1

// Put your code here.

(LOOP)
    @KBD
    D=M
    @WHITE
    D;JEQ
    @BLACK
    0;JMP

(WHITE)
    @black
    D=M
    @LOOP
    D;JEQ
    D=0
    @black
    M=D
    //draw white
    @white_or_black
    M=0
    @DRAW
    0;JMP

(BLACK)
    @black
    M=1
    //draw black
    @white_or_black
    M=-1
    @DRAW
    0;JMP

(DRAW)
    @8192
    D=A

    @counter
    M=D

    @SCREEN
    D=A

    @screenAddr
    M=D

    (INNER_LOOP)
    @white_or_black
    D=M
    @DRAW_WHITE
    D;JEQ
    @DRAW_BLACK
    D;JLE

    (POST_DRAW)
        @screenAddr
        M=D

        @counter
        D=M-1
        M=D

        @INNER_LOOP
        D;JNE

    @LOOP
    0;JMP

(DRAW_BLACK)
    @screenAddr
    D=M
    @SCREEN
    A=D
    M=-1
    D=D+1
    @POST_DRAW
    0;JMP

(DRAW_WHITE)
    @screenAddr
    D=M
    @SCREEN
    A=D
    M=0
    D=D+1
    @POST_DRAW
    0;JMP



