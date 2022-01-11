test_label:
    add #$10, %rax
    nop
    nop
    nop
    nop
    nop
    test_label.inner:
        inc %rax



_start:
    mov %rax, #$0
    mov %rdx, @msg
    int #$80, @ln
    xor %rax, %rax
    int #$80

.section rodata
    msg: .asciiz "Hello, World!"