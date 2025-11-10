.text
    ; Test CMPW and conditional jumps
    MOVI AX, 10.0
    MOVI BX, 5.0
    CMPW AX, BX      ; AX > BX, Zero=0, Sign=0

    JGT GreaterThan  ; Should jump
    JMP EndTest      ; Should not jump

GreaterThan:
    MOVI CX, 1       ; CX = 1 (Indicates JGT worked)

    MOVI AX, 5.0
    MOVI BX, 10.0
    CMPW AX, BX      ; AX < BX, Zero=0, Sign=1

    JLT LessThan     ; Should jump
    JMP EndTest      ; Should not jump

LessThan:
    MOVI CX, 2       ; CX = 2 (Indicates JLT worked)

    MOVI AX, 10.0
    MOVI BX, 10.0
    CMPW AX, BX      ; AX == BX, Zero=1, Sign=0

    JE EqualTo       ; Should jump
    JMP EndTest      ; Should not jump

EqualTo:
    MOVI CX, 3       ; CX = 3 (Indicates JE worked)

    MOVI AX, 10.0
    MOVI BX, 5.0
    CMPW AX, BX      ; AX > BX, Zero=0, Sign=0
    JNE NotEqual     ; Should jump
    JMP EndTest      ; Should not jump

NotEqual:
    MOVI CX, 4       ; CX = 4 (Indicates JNE worked)

    ; Test JMP
    JMP TargetJMP

TargetJMP:
    MOVI DX, 1       ; DX = 1 (Indicates JMP worked)

    ; Test CALL and RET
    CALL Subroutine
    MOVI DX, 3       ; DX = 3 (Should execute after RET)
    JMP EndTest

Subroutine:
    MOVI DX, 2       ; DX = 2 (Indicates CALL worked)
    RET

EndTest:
    HALT
