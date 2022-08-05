[org 0x7c00]

; entrypoint of the OS
main:
    call print_alphabet

    jmp $

print_alphabet:
    mov ah, 0x00e ; move to TTY mode
    mov al, 'a' ; the first letter of the alphabet
    jmp print_upper

print_upper:
    ; make `al` uppercase
    sub al, 32
    ; print `al`
    int 0x10
    ; print the lowercase version
    jmp print_lower

print_lower:
    ; make `al` lowercase
    add al, 32
    ; print `al`
    int 0x10
    ; go to next character
    inc al
    ; check if `al` is a letter
    cmp al, 'z' + 1
    ; if so, print the uppercase version
    jl print_upper
    ; if not, return
    ret

; finish the boot sector
times 510 - ($ - $$) db 0x00
db 0x55, 0xaa
