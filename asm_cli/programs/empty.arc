; New ARC Program
.text
    LODW AX [empty_word]  
    HALT               ; Stop execution
.data
    ; Data section (optional)
    empty_word:   .word  0x0120