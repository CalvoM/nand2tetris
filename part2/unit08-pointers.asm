//Code to run:
//for(i=0;i<n;i++)
//    arr[i] = -1

// program setup
//RAM[0] -> n
@R0
D=M
@n
M=D
// i = 0
@i
M=0
//Set the arr address
@100
D=A
@arr
M=D

// loop part
(loop)
// while i < n
@i
D=M
@n
D=D-M
@end
D;JEQ

@i
D=M
@arr
A=D+M
M=-1

//i++
@i
D=M
D=D+1
M=D

@loop
0;JMP

(end)
@end
0;JMP

