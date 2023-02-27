.section .boot, "awx"
.global _start
.code16

_start:
    mov sp, _stack_start
    mov ah, 0x0e
    mov al, '!'
    int 0x10

