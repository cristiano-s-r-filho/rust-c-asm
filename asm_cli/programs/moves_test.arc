.text
    ; Test MOVI
    MOVI AX, 123     ; AX = 123.0

    ; Test MOVW (register to register)
    MOVI BX, 456.7
    MOVW CX, BX      ; CX = 456.7

    ; Test MOVW (immediate to register)
    MOVW DX, 789     ; DX = 789.0

    ; Test LODI
    LODI EX, 1000    ; EX = 1000.0

    ; Test LODW (address to register)
    .data
    .word 12345      ; Value at 0x8000
    .text
    LODW FX, [0x8000] ; FX = 12345.0

    ; Test STRI (immediate to address)
    STRI [0x8004], 54321 ; Memory at 0x8004 = 54321.0

    ; Test STRW (register to address)
    MOVI GX, 987.6
    STRW [0x8008], GX ; Memory at 0x8008 = 987.6

    ; Test PUSH and POP
    MOVI AX, 1.0
    MOVI BX, 2.0
    PUSH AX          ; Push 1.0
    PUSH BX          ; Push 2.0
    POP CX           ; CX = 2.0
    POP DX           ; DX = 1.0

    ; Test XCGH
    MOVI AX, 10.0
    MOVI BX, 20.0
    XCGH AX, BX      ; AX = 20.0, BX = 10.0

    HALT
