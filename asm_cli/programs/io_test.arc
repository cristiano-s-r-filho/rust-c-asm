; This test file demonstrates basic I/O operations using the ARC assembly language.
; It includes examples of outputting immediate values, words from memory, and strings.
; It also includes examples of inputting words and strings into memory.

.data
my_word:    .word 0x12345678
my_string:  .string "Hello, World!"
in_word_dest: .space 4 ; Space for INSW to store a 32-bit word
in_string_destination: .space 16 ; Space for IN to store a string (e.g., up to 15 chars + null terminator)

.text
    ; Test OUTI - output immediate value
    OUTI 10          ; Output decimal 10
    OUTI 0xFF        ; Output hexadecimal 255

    ; Test INSI - input immediate value into a register
    ; The emulator will provide a value for INSI.
    MOVI AX, 0       ; Clear AX before input
    INSI 0           ; Input immediate (placeholder address, behavior needs clarification)

    ; Test OUTW - output word from memory
    OUTW my_word        ; Output word from the address in AX

    ; Test INSW - input word into memory
    ; The emulator will provide a value for INSW.
    INSW in_word_dest        ; Input word into the address in AX

    ; Test OUT - output string from memory
    OUT my_string

    ; Test IN - input string into memory
    IN in_string_destination    ; Input string into the address in AX

    HALT