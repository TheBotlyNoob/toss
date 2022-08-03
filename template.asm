[org 0x7c00]

; entrypoint of the OS
main:
    ; initialize the stack
    mov bp, 0x8000
    mov sp, bp
    
    jmp $

; finish the boot sector
times 510 - ($ - $$) db 0x00
db 0x55, 0xaa
