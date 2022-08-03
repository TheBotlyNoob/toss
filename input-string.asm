[org 0x7c00]

; entrypoint of the OS
main:
    ; initialize the stack
    mov bp, 0x8000
    mov sp, bp

    mov cx, 0
    
    mov ah, 0x00
    int 0x16
    cmp al, `\r`
    je on_newline
    mov ah, 0x00e
    int 0x10
    push ax
    inc cx
    jmp main

on_newline:
    call newline
    call print_stack
    jmp main

print_stack:
    pop ax
    dec cx
    mov ah, 0x00e
    int 0x10
    cmp cx, 0
    jne print_stack
    ret

newline:
    pusha
    mov ah, 0x00e
    mov al, `\r`
    int 0x10
    mov al, `\n`
    int 0x10
    popa
    ret

; finish the boot sector
times 510 - ($ - $$) db 0x00
db 0x55, 0xaa
