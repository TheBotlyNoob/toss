[org 0x7c00]

main:
    mov ah, 0x00
    int 0x16
    cmp al, `\r`
    je newline
    mov ah, 0x00e
    int 0x10
    jmp main

newline:
    mov ah, 0x00e
    mov al, `\r`
    int 0x10
    mov al, `\n`
    int 0x10
    jmp main

times 510 - ($ - $$) db 0
db 0x55, 0xaa
