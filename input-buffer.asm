
[org 0x7c00]

BUF_LEN equ 10
BUF: times BUF_LEN db 0

mov bx, BUF

; entrypoint of the OS
main:
    mov ah, 0x00
    int 0x16
    mov ah, 0x00e
    int 0x10

    mov [bx], al
    inc bx

    cmp bx, BUF + BUF_LEN
    je print_buf

    jmp main

print_buf:
    inc 
    mov bx, BUF
    mov ah, 0x00e

    ; newline
    mov al, `\r`
    int 0x10
    mov al, `\n`
    int 0x10

    l:
        mov al, [bx]
        int 0x10
        inc bx

        cmp bx, BUF + BUF_LEN
        jne l
        jmp $

; finish the boot sector
times 510 - ($ - $$) db 0
db 0x55, 0xaa
