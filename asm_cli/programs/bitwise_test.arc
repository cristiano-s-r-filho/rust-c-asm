.text
    ; Test AND
    MOVI AX, 0b1100  ; AX = 12
    MOVI BX, 0b1010  ; BX = 10
    AND AX, BX       ; AX = 0b1000 (8)
    AND AX, 0b0011   ; AX = 0b0000 (0)

    ; Test OR
    MOVI AX, 0b1100  ; AX = 12
    MOVI BX, 0b0011  ; BX = 3
    OR AX, BX        ; AX = 0b1111 (15)
    OR AX, 0b0101    ; AX = 0b1111 (15)

    ; Test XOR
    MOVI AX, 0b1100  ; AX = 12
    MOVI BX, 0b1010  ; BX = 10
    XOR AX, BX       ; AX = 0b0110 (6)
    XOR AX, 0b0110   ; AX = 0b0000 (0)

    ; Test NOT
    MOVI AX, 0b00000000000000000000000000001100 ; AX = 12
    NOT AX           ; AX = !12 (bitwise NOT)

    ; Test SHL
    MOVI AX, 1       ; AX = 1
    SHL AX, 2        ; AX = 4
    MOVI BX, 3       ; BX = 3
    SHL AX, BX       ; AX = 32

    ; Test SHR
    MOVI AX, 32      ; AX = 32
    SHR AX, 2        ; AX = 8
    MOVI BX, 3       ; BX = 3
    SHR AX, BX       ; AX = 1

    HALT
