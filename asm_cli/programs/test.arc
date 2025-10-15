; This is a test program for the ARC-0 emulator
; It tests all the instructions at least once.

.text
; Moves
MOVI AX, 10.0
MOVI BX, 5.0
MOVW CX, AX
XCGH AX, BX
PUSH AX
POP DX
LODI HX, 100
STRI [200], 20
LODW GX, [200]

; Arithmetic
ADDW AX, BX
SUBW AX, 1.0
INC AX
DEC AX
MUL AX, 2.0
NEG AX

; Bitwise
MOVI AX, 0b1010
MOVI BX, 0b1100
AND AX, BX
OR AX, 0b0011
XOR AX, 0b1111
NOT AX

; Compare and Jump
CMPW AX, BX
JMP end
JE end
JNE end
JGT end
JGE end
JLT end
JLE end
JS end
JCO end

; IO
IN AX
OUT AX

; Subroutine
CALL subroutine
HALT

subroutine:
    ADDW AX, 1.0
    RET

end:
HALT
