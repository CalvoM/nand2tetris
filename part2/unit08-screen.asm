@R0
D=M

@counter
M=D

@SCREEN
D=A

@screenAddr
M=D

(loop)
@screenAddr
D=M

@SCREEN
A=D
M=-1

@32
D=D+A

@screenAddr
M=D

@counter
D=M-1
M=D

@loop
D;JNE

(end)
@end
0;JMP
