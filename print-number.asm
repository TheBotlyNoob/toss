[org 0x7c00]

; entrypoint of the OS
main:
    ; initialize the stack pointer
    mov bp, 0x8000
    mov sp, bp
    
    mov ax, 12333 ; number to be printed

    jmp print_number

; print the number in `ax` as a decimal number
print_number:
    mov bx, 10 ; set the divisor to 10

    mov dx, 0 ; we need this why?
    div bx ; divide `ax` number by 10
    ; ax = quotient, dx = remainder

    push dx ; push the remainder to the stack

    cmp ax, 0 ; check if the quotient is 0 
    je print_number_end
    jmp print_number

print_number_end:
    ; print the number in the stack as a decimal number
    pop ax
    add al, '0' ; turn the number to ASCII

    mov ah, 0x0e
    int 0x10 ; print the number

    cmp sp, bp ; check if the stack is empty
    ; if it's not, loop
    jne print_number_end
    ; if it is, print a newline and exit
    mov al, `\r`
    int 0x10
    mov al, `\n`
    int 0x10
    jmp $


times 510 - ($ - $$) db 0 ; finish the boot sector
db 0x55, 0xaa
