[org 0x7c00]

; entrypoint of the OS
main:
    ; initialize the stack pointer
    mov bp, 0x8000
    mov sp, bp
    
    mov cx, 0 ; set the digit counter to 0
    mov ax, 125 ; number to be printed

    call print_number
    jmp exit

; print the number in `ax` as a decimal number
print_number:
    mov bx, 10 ; set divisor to 10

    mov dx, 0 ; reset the quotient
    div bx ; ax / bx = dx ; ax % bx = ax

    push dx ; push the remainder to the stack
    inc cx ; increment the digit counter

    cmp ax, 0 ; check if the remainder is zero
    je print_number_end ; if so, print the number and return
    jmp print_number ; if not, loop

print_number_end:
    pop ax ; pop the remainder from the stack
    dec cx ; decrement the counter

    add al, '0' ; make it an ASCII character
    push cx ; prevent the counter from being overwritten
    call print_char
    pop cx ; restore the counter
    cmp cx, 0 ; check if the counter is 0
    jne print_number_end ; loop if it is not

    ; if it is, print a newline and return
    call newline
    ret

newline:
    pusha
    mov al, `\r`
    call print_char
    mov al, `\n`
    call print_char
    popa
    ret

print_char:
    pusha
    mov ah, 0x0e
    int 0x10
    popa
    ret

; halt the OS and CPU
exit:
    hlt
    jmp exit

times 510 - ($ - $$) db 0 ; finish the boot sector
db 0x55, 0xaa
