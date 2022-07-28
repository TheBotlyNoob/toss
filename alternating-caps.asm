[org 0x7c00]

mov ah, 0x00e
mov al, 'a'

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
    ; if not, we're done
    jmp $

times 510 - ($ - $$) db 0
db 0x55, 0xaa
