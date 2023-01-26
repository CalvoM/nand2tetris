// This file is part of www.nand2tetris.org
// and the book "The Elements of Computing Systems"
// by Nisan and Schocken, MIT Press.
// File name: projects/04/Mult.asm

// Multiplies R0 and R1 and stores the result in R2.
// (R0, R1, R2 refer to RAM[0], RAM[1], and RAM[2], respectively.)
//
// This program only needs to handle arguments that satisfy
// R0 >= 0, R1 >= 0, and R0*R1 < 32768.

//Pseudocode
// Get number in R[1] and R[0] make sure > 0
// Get number in R[0] store in i
// Set R[2] to 0
// Loop
// if i is == 0 go to @end
// add R[1] number with itself save the number to R[2]
// reduce i by 1
// goto loop
// Put your code here.

//confirm R[1] > 0
@R1
D=M

@END
D;JLT

//confirm R[0] > 0
@R0
D=M

@END
D;JLT

@i
M=D

@R2
M=0

(LOOP)
    // if i ==0 jump to end
    @i
    D=M
    @END
    D;JEQ
    // get R1
    @R1
    D=M

    // update product
    @2
    A=M
    D=A+D

    // update R[2]
    @R2
    M=D


    // i--
    @i
    D=M
    D=D-1
    M=D

    @LOOP
    0;JMP

(END)
@END
0;JMP
