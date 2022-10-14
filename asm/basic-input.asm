[org 0x7c00]

; entrypoint of the OS
main:
    mov ah, 0x00
    int 0x16 ; get a character from the keyboard
    cmp al, `\r`
    je newline ; if they pressed enter, print a newline
    mov ah, 0x00e
    int 0x10
    jmp main
    
    jmp $

newline:
    mov ah, 0x00e
    mov al, `\r`
    int 0x10
    mov al, `\n`
    int 0x10
    jmp main

; finish the boot sector
times 510 - ($ - $$) db 0x00
db 0x55, 0xaa
