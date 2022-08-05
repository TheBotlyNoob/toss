[org 0x7c00]

; initialize the stack
mov bp, 0x8000
mov sp, bp

; entrypoint of the OS
main:
    jmp $

; finish the boot sector
times 510 - ($ - $$) db 0
db 0x55, 0xaa
