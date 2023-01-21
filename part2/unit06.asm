// Adds up two numbers 
// Usage: put the values that you wish to add 
// in RAM[0] and RAM[1]
// RAM[2] = RAM[0] + RAM[1]

@1
D=M

@0
D=D+M

@2
M=D

@SCREEN
A=A+320
M=16
