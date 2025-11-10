.text
    ; Test ADDW
    MOVI AX 10.0
    MOVI BX 5.0
    ADDW AX BX      ; AX = 15.0
    MOVI CX 2.5
    ADDW AX CX      ; AX = 17.5
    ADDW AX 7.5     ; AX = 25.0

    ; Test SUBW
    MOVI AX 30.0
    MOVI BX 5.0
    SUBW AX BX      ; AX = 25.0
    MOVI CX 2.5
    SUBW AX CX      ; AX = 22.5
    SUBW AX 2.5     ; AX = 20.0

    ; Test MUL
    MOVI AX 5.0
    MOVI BX 4.0
    MUL AX BX       ; AX = 20.0
    MOVI CX 2.0
    MUL AX CX       ; AX = 40.0
    MUL AX 0.5      ; AX = 20.0

    ; Test INC
    MOVI AX 9.0
    INC AX           ; AX = 10.0

    ; Test DEC
    MOVI AX 11.0
    DEC AX           ; AX = 10.0

    ; Test NEG
    MOVI AX 10.0
    NEG AX           ; AX = -10.0
    NEG AX           ; AX = 10.0
    HALT