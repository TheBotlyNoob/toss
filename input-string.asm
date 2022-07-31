[org 0x7c00]

main:
    mov bp, 0x8000
    mov sp, bp
    
    mov ah, 0x00
    int 0x16
    cmp al, `\r`
    je on_newline
    mov ah, 0x00e
    int 0x10
    push ax
    jmp main

on_newline:
    call newline
    call print_stack
    jmp main

print_stack:
    pop ax
    mov ah, 0x00e
    int 0x10
    cmp sp, bp
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

times 510 - ($ - $$) db 0
db 0x55, 0xaa
